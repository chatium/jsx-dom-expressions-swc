//! Times the pass over a corpus of already-lowered sources, for profiling and for checking
//! that an optimisation actually helped.
//!
//! ```bash
//! cargo run --release --example bench -- <dir of .jsx files> [iterations]
//! ```
//!
//! It reports parse and transform separately: only the transform is this crate's work, and
//! without the split a change that does nothing looks like a 30% win.

use std::path::Path;
use std::time::Instant;

use jsx_dom_expressions::{Config, Errors, jsx_dom_expressions};
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::sync::Lrc;
use swc_core::common::{FileName, GLOBALS, Globals, Mark, SourceMap};
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::transforms::base::resolver;

// swc_malloc registers the global allocator itself; linking it is the whole configuration.
use swc_malloc as _;

fn main() {
    let dir = Path::new(&std::env::args().nth(1).expect("corpus directory")).to_path_buf();
    let iterations: usize = std::env::args()
        .nth(2)
        .and_then(|n| n.parse().ok())
        .unwrap_or(3);

    let mut sources: Vec<(String, String)> = std::fs::read_dir(&dir)
        .expect("corpus directory")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "jsx"))
        .filter_map(|path| {
            let name = path.file_name()?.to_string_lossy().to_string();
            Some((name, std::fs::read_to_string(&path).ok()?))
        })
        .collect();
    sources.sort();
    let bytes: usize = sources.iter().map(|(_, text)| text.len()).sum();
    println!(
        "{} files, {:.1} MiB, {iterations} iterations",
        sources.len(),
        bytes as f64 / (1024.0 * 1024.0)
    );

    // Parse everything once so the profile is not dominated by the parser: the loop then
    // clones a program and transforms it, which is the shape a host compiler runs.
    if std::env::var("BENCH_TRANSFORM_ONLY").is_ok() {
        let parsed: Vec<_> = sources
            .iter()
            .filter_map(|(name, source)| {
                let cm: Lrc<SourceMap> = Default::default();
                let fm = cm.new_source_file(FileName::Custom(name.clone()).into(), source.clone());
                let comments = SingleThreadedComments::default();
                let mut parser = Parser::new(
                    Syntax::Es(EsSyntax {
                        jsx: true,
                        ..Default::default()
                    }),
                    StringInput::from(&*fm),
                    Some(&comments),
                );
                Some((parser.parse_program().ok()?, comments))
            })
            .collect();
        let mut total = 0f64;
        for _ in 0..iterations {
            let started = Instant::now();
            for (program, comments) in &parsed {
                GLOBALS.set(&Globals::default(), || {
                    let unresolved = Mark::new();
                    let top_level = Mark::new();
                    let _ = program
                        .clone()
                        .apply(resolver(unresolved, top_level, false))
                        .apply(jsx_dom_expressions(
                            Config {
                                module_name: "solid-js/web".into(),
                                ..Default::default()
                            },
                            comments,
                            Errors::default(),
                        ));
                });
            }
            let elapsed = started.elapsed().as_secs_f64();
            total += elapsed;
            println!("  transform {elapsed:.3}s");
        }
        println!(
            "mean transform-only: {:.3}s ({:.1} MiB/s)",
            total / iterations as f64,
            bytes as f64 / (1024.0 * 1024.0) / (total / iterations as f64)
        );
        return;
    }

    let mut parse_total = 0f64;
    let mut transform_total = 0f64;
    for _ in 0..iterations {
        let mut parsed = 0f64;
        let mut transformed = 0f64;
        for (name, source) in &sources {
            let started = Instant::now();
            let cm: Lrc<SourceMap> = Default::default();
            let fm = cm.new_source_file(FileName::Custom(name.clone()).into(), source.clone());
            let comments = SingleThreadedComments::default();
            let mut parser = Parser::new(
                Syntax::Es(EsSyntax {
                    jsx: true,
                    ..Default::default()
                }),
                StringInput::from(&*fm),
                Some(&comments),
            );
            let Ok(program) = parser.parse_program() else {
                continue;
            };
            parsed += started.elapsed().as_secs_f64();

            let started = Instant::now();
            GLOBALS.set(&Globals::default(), || {
                let unresolved = Mark::new();
                let top_level = Mark::new();
                let _ = program.apply(resolver(unresolved, top_level, false)).apply(
                    jsx_dom_expressions(
                        Config {
                            module_name: "solid-js/web".into(),
                            ..Default::default()
                        },
                        &comments,
                        Errors::default(),
                    ),
                );
            });
            transformed += started.elapsed().as_secs_f64();
        }
        parse_total += parsed;
        transform_total += transformed;
        println!("  parse {parsed:.3}s   transform {transformed:.3}s");
    }
    let n = iterations as f64;
    println!(
        "mean: parse {:.3}s  transform {:.3}s  ({:.1} MiB/s through the pass)",
        parse_total / n,
        transform_total / n,
        bytes as f64 / (1024.0 * 1024.0) / (transform_total / n),
    );
}
