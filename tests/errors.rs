//! The diagnostics the Babel plugin raises by throwing. Each expectation here was checked
//! against the real plugin (`tools/`), message text included.

mod harness;

use harness::{compile_collecting, dom_config};
use jsx_dom_expressions::Config;

fn messages(source: &str) -> Vec<String> {
    compile_collecting(source, dom_config())
        .1
        .into_iter()
        .map(|error| error.message)
        .collect()
}

#[test]
fn rejects_invalid_html_nesting() {
    assert_eq!(
        messages("const a = <p><div/></p>;"),
        ["Invalid JSX: <div> cannot be child of <p>"]
    );
    assert_eq!(
        messages("const a = <table><tr/></table>;"),
        ["Invalid JSX: <tr> cannot be child of <table>"]
    );
}

/// Babel checks `path.parent`, so only a *directly* nested element is validated. Anything
/// behind an expression container, an attribute or another element is not its business.
#[test]
fn only_direct_children_are_validated() {
    assert!(messages("const a = <p>{<div/>}</p>;").is_empty());
    assert!(messages("const a = <h1 onClick={() => f(<h1>x</h1>)}>y</h1>;").is_empty());
    assert!(messages("const a = <p><span><div/></span></p>;").is_empty());
    // …and the direct case still is.
    assert_eq!(
        messages("const a = <p><div/></p>;"),
        ["Invalid JSX: <div> cannot be child of <p>"]
    );
}

#[test]
fn accepts_valid_nesting_and_components() {
    assert!(messages("const a = <ul><li/></ul>;").is_empty());
    // A component's tag says nothing about the HTML it renders.
    assert!(messages("const a = <p><Comp/></p>;").is_empty());
    assert!(messages("const a = <Comp><div/></Comp>;").is_empty());
}

#[test]
fn rejects_a_fragment_nested_in_an_element() {
    assert_eq!(
        messages("const a = <div><><span/></></div>;"),
        ["Fragments can only be used top level in JSX. Not used under a <div>."]
    );
}

#[test]
fn validation_can_be_turned_off() {
    let mut config = dom_config();
    config.validate = false;
    assert!(
        compile_collecting("const a = <p><div/></p>;", config)
            .1
            .is_empty()
    );
}

/// A deliberate divergence. A JSX string may span lines, which a JS string literal may not.
/// `transformComponent` upstream rebuilds the literal so Babel's printer escapes it, but
/// `processSpreads` passes the JSX node straight through, and Babel then prints the raw source
/// bytes — output that does not parse back. We always rebuild, so ours does.
#[test]
fn multiline_jsx_strings_survive_becoming_js_literals() {
    for source in [
        "const a = <div {...rest} style=\"x:1;\ny:2\" />;",
        "const a = <Card label=\"line one\nline two\" />;",
        // The shape of a real source whose output did not parse back.
        "const a = <Flex column gap={5} style=\"\n\
         padding: 5px 0 5px 10px;\n\
         margin-left: 17px;\n\
         color: rgb(0 0 0);\n\
         border-left: 3px solid #ffab00;\n\
         \">x</Flex>;",
    ] {
        let (out, errors) = compile_collecting(source, dom_config());
        assert!(errors.is_empty(), "{errors:?}");
        assert!(
            !out.contains("\"x:1;\n") && !out.contains("\"line one\n"),
            "a raw newline survived into a string literal: {out}"
        );
        harness::parses(&out);
    }
}

#[test]
fn require_import_source_skips_files_without_the_pragma() {
    let mut config = dom_config();
    config.require_import_source = Some("solid-js".into());
    let (without, _) = compile_collecting("const a = <div/>;", config.clone());
    assert!(
        !without.contains("_$template"),
        "a file with no pragma is left alone, got: {without}"
    );
    let (with, _) = compile_collecting(
        "/* @jsxImportSource solid-js */\nconst a = <div/>;",
        config.clone(),
    );
    assert!(with.contains("_$template"), "got: {with}");
    // A pragma that is not the first thing in the file is found by scanning the statements,
    // and a Module scans a different list from a Script.
    let (module, _) = compile_collecting(
        "import { x } from \"y\";\n/* @jsxImportSource solid-js */\nconst a = <div>{x}</div>;",
        config,
    );
    assert!(module.contains("_$template"), "got: {module}");
}

/// Nothing in the pass names a runtime: the same input retargets by config alone.
#[test]
fn the_runtime_module_is_configuration_not_code() {
    let source = "const a = <div class={c()}>{x()}</div>;";
    for module in [
        "dom",
        "r-dom",
        "solid-js/web",
        "solid-js/web",
        "my/own/runtime",
    ] {
        let (out, errors) = compile_collecting(
            source,
            Config {
                module_name: module.into(),
                ..Default::default()
            },
        );
        assert!(errors.is_empty(), "{errors:?}");
        let imports = out.lines().filter(|l| l.starts_with("import ")).count();
        assert!(imports > 0, "{module}: nothing imported\n{out}");
        assert_eq!(
            out.matches(&format!("from \"{module}\"")).count(),
            imports,
            "{module}: every generated import must come from it\n{out}"
        );
    }
}

/// The effect and memo wrappers are named by config too, not baked in.
#[test]
fn the_reactive_wrappers_are_configuration_too() {
    let source = "const a = <div title={t()}>{cond() ? <b/> : <i/>}</div>;";
    let (out, _) = compile_collecting(
        source,
        Config {
            module_name: "rt".into(),
            effect_wrapper: "myEffect".into(),
            memo_wrapper: "myMemo".into(),
            ..Default::default()
        },
    );
    assert!(
        out.contains(r#"import { myMemo as _$myMemo } from "rt""#),
        "{out}"
    );
    assert!(
        out.contains(r#"import { myEffect as _$myEffect } from "rt""#),
        "{out}"
    );
    assert!(
        !out.contains("_$memo") && !out.contains("_$effect"),
        "{out}"
    );
}
