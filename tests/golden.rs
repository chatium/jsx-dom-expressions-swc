//! Every upstream fixture, compiled by this pass and compared with the real Babel plugin's
//! output for the same input. Both sides are printed by SWC, so only the AST has to match.

mod harness;

use harness::{
    compile, compile_in_pipeline, config_from, configs, normalize, normalize_in_pipeline, root,
};
use jsx_dom_expressions::Config;

fn run(config_name: &str) {
    run_with(config_name, compile, normalize);
}

/// The same comparison with `hygiene` and `fixer` applied to both sides, which is what the
/// consumer does to the Node helper's output today.
fn run_in_pipeline(config_name: &str) {
    run_with(config_name, compile_in_pipeline, normalize_in_pipeline);
}

fn run_with(
    config_name: &str,
    compile: fn(&str, &str, Config) -> String,
    normalize: fn(&str, &str) -> String,
) {
    let manifest = configs();
    let entry = manifest
        .get(config_name)
        .unwrap_or_else(|| panic!("no `{config_name}` in tools/configs.json"));
    let suite = entry["fixtures"].as_str().expect("fixtures name");
    let options = &entry["options"];
    let cases = root().join("tests/cases").join(suite);
    let mut failures = Vec::new();
    let mut total = 0;
    let mut groups: Vec<_> = std::fs::read_dir(&cases)
        .unwrap_or_else(|error| panic!("{}: {error}", cases.display()))
        .map(|entry| entry.unwrap().path())
        .collect();
    groups.sort();
    for group in groups {
        let mut files: Vec<_> = std::fs::read_dir(&group)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "jsx"))
            .collect();
        files.sort();
        for file in files {
            total += 1;
            let name = format!(
                "{}/{}",
                group.file_name().unwrap().to_string_lossy(),
                file.file_stem().unwrap().to_string_lossy()
            );
            if let Ok(only) = std::env::var("JSX_ONLY")
                && !name.contains(&only)
            {
                total -= 1;
                continue;
            }
            let source = std::fs::read_to_string(&file).unwrap();
            let expected_path = root()
                .join("tests/expected")
                .join(config_name)
                .join(format!("{name}.js"));
            let expected_source = std::fs::read_to_string(&expected_path).unwrap();
            let expected = normalize(&name, &expected_source);
            let actual = std::panic::catch_unwind(|| compile(&name, &source, config_from(options)));
            match actual {
                Ok(actual) if actual == expected => {}
                Ok(actual) => failures.push((name, expected, actual)),
                Err(_) => failures.push((name, expected, "<panicked>".into())),
            }
        }
    }
    if !failures.is_empty() {
        let shown = failures
            .iter()
            .take(std::env::var("JSX_ONLY").map(|_| 20).unwrap_or(3))
            .map(|(name, expected, actual)| {
                format!("--- {name}\nexpected:\n{expected}\nactual:\n{actual}")
            })
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "{}/{} fixtures differ for `{config_name}`:\n{}\n\nfailing: {}",
            failures.len(),
            total,
            shown,
            failures
                .iter()
                .map(|(name, _, _)| name.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

#[test]
fn app_dom() {
    run("app-dom");
}

#[test]
fn app_dom_through_the_host_pipeline() {
    run_in_pipeline("app-dom");
}

#[test]
fn app_ssr_through_the_host_pipeline() {
    run_in_pipeline("app-ssr");
}

#[test]
fn app_ssr() {
    run("app-ssr");
}

/// Sources shaped like real application code, beyond what the upstream corpus covers.
#[test]
fn usage_dom() {
    run("usage-dom");
    run_in_pipeline("usage-dom");
}

#[test]
fn usage_ssr() {
    run("usage-ssr");
    run_in_pipeline("usage-ssr");
}

/// The same realistic sources under hydration, which pairs a server render with a client
/// that adopts it: both generators change, so both are checked.
#[test]
fn usage_dom_hydratable() {
    run("usage-dom-hydratable");
    run_in_pipeline("usage-dom-hydratable");
}

#[test]
fn usage_ssr_hydratable() {
    run("usage-ssr-hydratable");
    run_in_pipeline("usage-ssr-hydratable");
}

/// Options no upstream spec file switches on: `omitNestedClosingTags`, a custom
/// `delegatedEvents` list and a non-default `staticMarker`.
#[test]
fn options_upstream_never_tests() {
    run("usage-untested-options");
    run_in_pipeline("usage-untested-options");
}

#[test]
fn upstream_dom() {
    run("spec");
}

#[test]
fn upstream_dom_hydratable() {
    run("dom-hydratable");
}

#[test]
fn upstream_dom_wrapperless() {
    run("dom-wrapperless");
}

#[test]
fn upstream_ssr() {
    run("ssr");
}

#[test]
fn upstream_ssr_hydratable() {
    run("ssr-hydratable");
}
