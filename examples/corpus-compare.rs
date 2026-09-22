//! Compiles a real UGC project through both Solid paths and compares them.
//!
//! This is the evidence behind the compatibility claim: the fixture suites are upstream's, but
//! only real sources exercise the combinations applications actually write. Four live bugs were
//! found this way and are pinned in `tests/cases/usage/app/07-regressions.jsx`.
//!
//! ```bash
//! # 1. lower TSX to JS with JSX preserved, the way a host compiler does before this pass
//! cargo run --release --example corpus-compare -- lower <project-dir> <out-dir> [filter]
//!
//! # 2. run the Babel plugin over them (from a checkout that has it installed)
//! node -e '...babel.transformSync(src, { plugins: [[solid, { moduleName: "solid-js/web" }]] })...'
//! #    writing each result next to its input as <name>.babel.js
//!
//! # 3. compare our pass against that, inside the consumer's hygiene/fixer pipeline
//! cargo run --release --example corpus-compare -- check <out-dir>
//! ```
//!
//! Differences are reported in three buckets. Only "genuine AST differences" is a bug; the
//! other two are unavoidable consequences of the current path printing to text and re-parsing.
use std::path::{Path, PathBuf};

use jsx_dom_expressions::{Config, Errors, jsx_dom_expressions};
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::sync::Lrc;
use swc_core::common::{FileName, GLOBALS, Globals, Mark, SourceMap};
use swc_core::ecma::ast::Program;
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax, TsSyntax};
use swc_core::ecma::transforms::base::fixer::fixer;
use swc_core::ecma::transforms::base::helpers::{HELPERS, Helpers, inject_helpers};
use swc_core::ecma::transforms::base::hygiene::hygiene;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::proposal::decorators::{Config as DecoratorConfig, decorators};
use swc_core::ecma::transforms::typescript::strip;

fn parse(
    path: &str,
    source: &str,
    syntax: Syntax,
) -> (Program, SingleThreadedComments, Lrc<SourceMap>) {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom(path.into()).into(), source.to_string());
    let comments = SingleThreadedComments::default();
    let mut parser = Parser::new(syntax, StringInput::from(&*fm), Some(&comments));
    let program = parser
        .parse_program()
        .unwrap_or_else(|e| panic!("{path}: {e:?}"));
    (program, comments, cm)
}

fn print(program: &Program, comments: &SingleThreadedComments, cm: Lrc<SourceMap>) -> String {
    print_opt(
        program,
        Some(comments as &dyn swc_core::common::comments::Comments),
        cm,
    )
}

fn print_bare(program: &Program, cm: Lrc<SourceMap>) -> String {
    print_opt(program, None, cm)
}

fn print_opt(
    program: &Program,
    comments: Option<&dyn swc_core::common::comments::Comments>,
    cm: Lrc<SourceMap>,
) -> String {
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Default::default(),
            comments,
            cm: cm.clone(),
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        emitter.emit_program(program).expect("emit");
    }
    String::from_utf8(buf).expect("utf8")
}

fn tsx() -> Syntax {
    Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    })
}

fn jsx() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

/// compiler.rs::solid_source — JS with JSX preserved.
fn lower(path: &str, source: &str) -> String {
    let (program, comments, cm) = parse(path, source, tsx());
    GLOBALS.set(&Globals::default(), || {
        HELPERS.set(&Helpers::new(false), || {
            let unresolved = Mark::new();
            let top_level = Mark::new();
            let program = program
                .apply(resolver(unresolved, top_level, true))
                .apply(decorators(DecoratorConfig {
                    legacy: true,
                    emit_metadata: false,
                    use_define_for_class_fields: false,
                }))
                .apply(strip(unresolved, top_level))
                .apply(inject_helpers(unresolved))
                .apply(hygiene())
                .apply(fixer(Some(&comments)));
            print(&program, &comments, cm)
        })
    })
}

/// The runtime the corpus was compiled against; must match what Babel was given.
fn runtime_module() -> String {
    std::env::var("CORPUS_MODULE").unwrap_or_else(|_| "solid-js/web".into())
}

/// `CORPUS_GENERATE=ssr` and `CORPUS_HYDRATABLE=1` mirror the same Babel options, so a corpus
/// can be checked against the SSR generator as well as the DOM one.
fn corpus_config() -> Config {
    Config {
        module_name: runtime_module(),
        generate: match std::env::var("CORPUS_GENERATE").as_deref() {
            Ok("ssr") => jsx_dom_expressions::Generate::Ssr,
            _ => jsx_dom_expressions::Generate::Dom,
        },
        hydratable: std::env::var("CORPUS_HYDRATABLE").is_ok(),
        ..Default::default()
    }
}

/// Our pass inside the consumer's pipeline.
fn ours(path: &str, source: &str) -> (String, Vec<String>) {
    let (program, comments, cm) = parse(path, source, jsx());
    let errors = Errors::default();
    GLOBALS.set(&Globals::default(), || {
        let unresolved = Mark::new();
        let top_level = Mark::new();
        let program = program
            .apply(resolver(unresolved, top_level, false))
            .apply(jsx_dom_expressions(
                corpus_config(),
                &comments,
                errors.clone(),
            ))
            .apply(hygiene())
            .apply(fixer(Some(&comments)));
        let out = print(&program, &comments, cm);
        (out, errors.take().into_iter().map(|e| e.message).collect())
    })
}

/// The Babel output as the consumer receives it today: re-parsed and lowered by SWC.
fn theirs(path: &str, source: &str) -> String {
    let (program, comments, cm) = parse(path, source, jsx());
    GLOBALS.set(&Globals::default(), || {
        let unresolved = Mark::new();
        let top_level = Mark::new();
        let program = program
            .apply(resolver(unresolved, top_level, false))
            .apply(hygiene())
            .apply(fixer(Some(&comments)));
        print(&program, &comments, cm)
    })
}

/// Re-print with comments dropped, to tell an AST difference from a comment-placement one.
fn strip_comments(path: &str, source: &str) -> String {
    let (program, _, cm) = parse(path, source, jsx());
    print_bare(&program, cm)
}

/// Same, but also dropping literal `raw` (Babel escapes non-ASCII where SWC emits the
/// character) and parens, so only a genuine AST difference survives.
fn normalize_ast(path: &str, source: &str) -> String {
    use swc_core::ecma::visit::{VisitMut, VisitMutWith};
    struct N;
    impl VisitMut for N {
        fn visit_mut_str(&mut self, n: &mut swc_core::ecma::ast::Str) {
            n.raw = None;
        }
        fn visit_mut_number(&mut self, n: &mut swc_core::ecma::ast::Number) {
            n.raw = None;
        }
        fn visit_mut_expr(&mut self, e: &mut swc_core::ecma::ast::Expr) {
            e.visit_mut_children_with(self);
            if let swc_core::ecma::ast::Expr::Paren(p) = e {
                *e = (*p.expr).clone();
            }
        }
    }
    let (mut program, _, cm) = parse(path, source, jsx());
    program.visit_mut_with(&mut N);
    print_bare(&program, cm)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .is_some_and(|n| n == "node_modules" || n == ".git")
            {
                continue;
            }
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "tsx" || e == "jsx") {
            out.push(path);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "lower" => {
            let root = Path::new(&args[2]);
            let out = Path::new(&args[3]);
            std::fs::create_dir_all(out).unwrap();
            let mut files = Vec::new();
            walk(root, &mut files);
            files.sort();
            // Optional substring filter over the source, e.g. a runtime import name; leaving it
            // off takes every file that still has JSX after lowering.
            let filter = args.get(4).cloned();
            let mut n = 0;
            for file in &files {
                let Ok(source) = std::fs::read_to_string(file) else {
                    continue;
                };
                if filter
                    .as_ref()
                    .is_some_and(|f| !source.contains(f.as_str()))
                {
                    continue;
                }
                let rel = file
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('/', "__");
                let lowered = match std::panic::catch_unwind(|| lower(&rel, &source)) {
                    Ok(text) => text,
                    Err(_) => {
                        eprintln!("PARSE FAIL {rel}");
                        continue;
                    }
                };
                if !lowered.contains('<') {
                    continue;
                }
                std::fs::write(out.join(format!("{rel}.jsx")), lowered).unwrap();
                n += 1;
            }
            println!("lowered {n} of {} .tsx files", files.len());
        }
        "check" => {
            let out = Path::new(&args[2]);
            let mut files: Vec<_> = std::fs::read_dir(out)
                .unwrap()
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.to_string_lossy().ends_with(".jsx"))
                .collect();
            files.sort();
            let (mut same, mut diff, mut errs, mut missing) = (0, Vec::new(), Vec::new(), 0);
            let mut comments_only = 0;
            let mut real: Vec<String> = Vec::new();
            let mut cosmetic = 0;
            for file in &files {
                let name = file.file_name().unwrap().to_string_lossy().to_string();
                let babel_path = file.with_extension("babel.js");
                if !babel_path.exists() {
                    missing += 1;
                    continue;
                }
                let lowered = std::fs::read_to_string(file).unwrap();
                let (mine, messages) = ours(&name, &lowered);
                if !messages.is_empty() {
                    errs.push((name.clone(), messages));
                }
                let babel = theirs(&name, &std::fs::read_to_string(&babel_path).unwrap());
                if mine == babel {
                    same += 1;
                } else {
                    if strip_comments(&name, &mine) == strip_comments(&name, &babel) {
                        comments_only += 1;
                    } else if normalize_ast(&name, &mine) == normalize_ast(&name, &babel) {
                        cosmetic += 1;
                    } else {
                        real.push(name.clone());
                    }
                    std::fs::write(file.with_extension("mine.js"), &mine).unwrap();
                    std::fs::write(file.with_extension("norm.js"), &babel).unwrap();
                    diff.push(name);
                }
            }
            println!(
                "byte-identical: {same}\n  differ only in comment placement: {comments_only}\n  \
                 differ only in literal escaping/parens: {cosmetic}\n  \
                 genuine AST differences: {}\n  missing babel output: {missing}",
                real.len()
            );
            for name in real.iter() {
                println!("  AST-DIFF {name}");
            }
            for (name, messages) in &errs {
                println!("  ERRORS {name}: {messages:?}");
            }
        }
        "dump" => {
            let source = std::fs::read_to_string(&args[2]).unwrap();
            let (out, errors) = ours("dump.jsx", &source);
            if errors.is_empty() {
                print!("{out}");
            } else {
                for message in errors {
                    println!("ERROR: {message}");
                }
            }
        }
        other => panic!("unknown mode {other}"),
    }
}
