//! Shared plumbing for the fixture and diagnostics tests: parse with SWC, run the pass,
//! print both sides with the same emitter so only the AST has to match.

// Each integration test binary uses a different slice of this module.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use jsx_dom_expressions::{Config, Errors, Generate, jsx_dom_expressions};
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::sync::Lrc;
use swc_core::common::util::take::Take;
use swc_core::common::{FileName, GLOBALS, Globals, Mark, SourceMap};
use swc_core::ecma::ast::{Expr, Program};
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::transforms::base::fixer::fixer;
use swc_core::ecma::transforms::base::hygiene::hygiene;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// The single source of truth for both sides: `tools/gen.mjs` feeds these same options to
/// the real Babel plugin.
pub fn configs() -> serde_json::Map<String, serde_json::Value> {
    let raw = std::fs::read_to_string(root().join("tools/configs.json")).expect("configs.json");
    serde_json::from_str::<serde_json::Value>(&raw)
        .expect("configs.json is valid json")
        .as_object()
        .expect("configs.json is an object")
        .clone()
}

pub fn config_from(options: &serde_json::Value) -> Config {
    let mut config = Config::default();
    let string = |value: Option<&serde_json::Value>, current: String| match value {
        Some(serde_json::Value::String(text)) => text.clone(),
        // Upstream spells "no wrapper" as `false`.
        Some(serde_json::Value::Bool(false)) => String::new(),
        _ => current,
    };
    let flag = |value: Option<&serde_json::Value>, current: bool| {
        value
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(current)
    };
    config.module_name = string(options.get("moduleName"), config.module_name);
    config.generate = match options.get("generate").and_then(serde_json::Value::as_str) {
        Some("ssr") => Generate::Ssr,
        _ => Generate::Dom,
    };
    config.hydratable = flag(options.get("hydratable"), config.hydratable);
    config.delegate_events = flag(options.get("delegateEvents"), config.delegate_events);
    config.wrap_conditionals = flag(options.get("wrapConditionals"), config.wrap_conditionals);
    config.omit_nested_closing_tags = flag(
        options.get("omitNestedClosingTags"),
        config.omit_nested_closing_tags,
    );
    config.context_to_custom_elements = flag(
        options.get("contextToCustomElements"),
        config.context_to_custom_elements,
    );
    config.validate = flag(options.get("validate"), config.validate);
    config.static_marker = string(options.get("staticMarker"), config.static_marker);
    config.effect_wrapper = string(options.get("effectWrapper"), config.effect_wrapper);
    config.memo_wrapper = string(options.get("memoWrapper"), config.memo_wrapper);
    for key in ["builtIns", "delegatedEvents"] {
        let values = options
            .get(key)
            .and_then(serde_json::Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if key == "builtIns" {
            config.built_ins = values;
        } else {
            config.delegated_events = values;
        }
    }
    config
}

pub fn parse(
    path: &str,
    source: &str,
    jsx: bool,
) -> (Program, SingleThreadedComments, Lrc<SourceMap>) {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom(path.into()).into(), source.to_string());
    let comments = SingleThreadedComments::default();
    let mut parser = Parser::new(
        Syntax::Es(EsSyntax {
            jsx,
            ..Default::default()
        }),
        StringInput::from(&*fm),
        Some(&comments),
    );
    let program = parser
        .parse_program()
        .unwrap_or_else(|error| panic!("{path}: {error:?}"));
    (program, comments, cm)
}

/// Canonicalises away printer-level differences before comparing:
/// Babel's AST has no parenthesised-expression node, so its output never carries a redundant
/// paren, and it escapes non-ASCII in string literals where SWC emits the character.
struct Normalize;

impl VisitMut for Normalize {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);
        if let Expr::Paren(paren) = expr {
            *expr = *paren.expr.take();
        }
    }

    fn visit_mut_str(&mut self, node: &mut swc_core::ecma::ast::Str) {
        node.raw = None;
    }

    fn visit_mut_number(&mut self, node: &mut swc_core::ecma::ast::Number) {
        node.raw = None;
    }
}

/// Prints exactly what the pass produced. Use this to judge the output itself; `print`
/// normalises for comparison and would hide, say, an unescaped literal.
pub fn print_raw(
    program: &Program,
    comments: &SingleThreadedComments,
    cm: Lrc<SourceMap>,
) -> String {
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Default::default(),
            comments: Some(comments),
            cm: cm.clone(),
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        emitter.emit_program(program).expect("emit");
    }
    String::from_utf8(buf).expect("utf8")
}

pub fn print(program: &Program, comments: &SingleThreadedComments, cm: Lrc<SourceMap>) -> String {
    let mut program = program.clone();
    program.visit_mut_with(&mut Normalize);
    let program = &program;
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Default::default(),
            comments: Some(comments),
            cm: cm.clone(),
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        emitter.emit_program(program).expect("emit");
    }
    String::from_utf8(buf).expect("utf8")
}

pub fn compile(path: &str, source: &str, config: Config) -> String {
    let (mut program, comments, cm) = parse(path, source, true);
    let errors = Errors::default();
    GLOBALS.set(&Globals::default(), || {
        let unresolved = Mark::new();
        let top_level = Mark::new();
        program = std::mem::replace(&mut program, Program::Script(Default::default()))
            .apply(resolver(unresolved, top_level, false))
            .apply(jsx_dom_expressions(config, &comments, errors.clone()));
        let printed = print(&program, &comments, cm);
        assert!(errors.is_empty(), "{path}: {:?}", errors.take());
        printed
    })
}

pub fn normalize(path: &str, source: &str) -> String {
    let (program, comments, cm) = parse(path, source, false);
    print(&program, &comments, cm)
}

/// Asserts the text is valid JavaScript, which is the whole point of rebuilding literals.
pub fn parses(source: &str) {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("check.js".into()).into(),
        source.to_string(),
    );
    let mut parser = Parser::new(
        Syntax::Es(Default::default()),
        StringInput::from(&*fm),
        None,
    );
    parser
        .parse_program()
        .unwrap_or_else(|error| panic!("output does not parse: {error:?}\n{source}"));
}

pub fn dom_config() -> Config {
    Config {
        module_name: "r-dom".into(),
        ..Default::default()
    }
}

/// The pipeline a host compiler runs around this pass: the JSX transform is followed by
/// `hygiene` and `fixer` before the emitter sees the program. Anything the pass gets wrong
/// about scopes or precedence shows up here and not in `compile`.
pub fn compile_in_pipeline(path: &str, source: &str, config: Config) -> String {
    let (mut program, comments, cm) = parse(path, source, true);
    let errors = Errors::default();
    GLOBALS.set(&Globals::default(), || {
        let unresolved = Mark::new();
        let top_level = Mark::new();
        program = std::mem::replace(&mut program, Program::Script(Default::default()))
            .apply(resolver(unresolved, top_level, false))
            .apply(jsx_dom_expressions(config, &comments, errors.clone()))
            .apply(hygiene())
            .apply(fixer(Some(&comments)));
        let printed = print(&program, &comments, cm);
        assert!(errors.is_empty(), "{path}: {:?}", errors.take());
        printed
    })
}

/// The Babel golden put through the same `hygiene`/`fixer` a host runs, which is what it is
/// subjected to when the Babel plugin's text is re-parsed and lowered by SWC.
pub fn normalize_in_pipeline(path: &str, source: &str) -> String {
    let (mut program, comments, cm) = parse(path, source, false);
    GLOBALS.set(&Globals::default(), || {
        let unresolved = Mark::new();
        let top_level = Mark::new();
        program = std::mem::replace(&mut program, Program::Script(Default::default()))
            .apply(resolver(unresolved, top_level, false))
            .apply(hygiene())
            .apply(fixer(Some(&comments)));
        print(&program, &comments, cm)
    })
}

/// Compiles and hands back whatever diagnostics the pass recorded.
pub fn compile_collecting(
    source: &str,
    config: Config,
) -> (String, Vec<jsx_dom_expressions::CompileError>) {
    let (mut program, comments, cm) = parse("test.jsx", source, true);
    let errors = Errors::default();
    GLOBALS.set(&Globals::default(), || {
        let unresolved = Mark::new();
        let top_level = Mark::new();
        program = std::mem::replace(&mut program, Program::Script(Default::default()))
            .apply(resolver(unresolved, top_level, false))
            .apply(jsx_dom_expressions(config, &comments, errors.clone()));
        (print_raw(&program, &comments, cm), errors.take())
    })
}
