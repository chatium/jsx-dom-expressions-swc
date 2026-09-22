# AGENTS.md — `jsx-dom-expressions`

A port of `babel-plugin-jsx-dom-expressions` 0.36.10 to an SWC pass. The upstream plugin is the
specification: when this code and the JS disagree, the JS wins, and the fixture goldens say so.

## Module graph — acyclic, one direction

```
lib ─▶ transform ─▶ element ─▶ template
        │   │           └────▶ constants, eval, utils
        │   ├──────▶ component
        │   └──────▶ ssr
        ├──▶ preprocess ─▶ nesting
        ├──▶ scope, uid, errors, config, entities
```

- `lib.rs` — the `VisitMut` entry point, program prologue/epilogue, pending-statement buffer.
- `transform.rs` — `transformJSX` / `transformNode` / `transformCondition`, `Results`, `Info`.
- `element.rs` — the DOM generator (`dom/element.js`), including attribute preprocessing.
- `ssr.rs` — the SSR generator (`ssr/element.js` + `ssr/template.js`).
- `component.rs` — `shared/component.js`, shared by both generators.
- `template.rs` — template registration, import registration, small AST builders.
- `utils.rs`, `eval.rs` — `shared/utils.js` and the `path.evaluate()` subset.
- `scope.rs`, `uid.rs` — the slices of Babel's scope this needs: bindings and `generateUid`.
- `preprocess.rs`, `nesting.rs` — `validate` and `requireImportSource`.
- `constants.rs`, `nesting.rs` tables — **generated**; edit `tools/gen-constants.mjs` instead.

## Rules

- Upstream's behavior is the contract, quirks included. Where a quirk is load-bearing —
  Babel's `generateUid` suffix sequence, its `splice` index arithmetic, the fact that imports
  print in reverse registration order — port it literally and say why in a comment.
- Where SWC's AST forces a divergence (no paren node, `@once` attached as a trailing comment,
  CRLF in JSX strings, `BinExpr` covering logical operators), handle it at the boundary and
  name the reason. These are the bugs that cost the most to rediscover.
- No abstraction with one implementation; SWC and stdlib before custom code.
- New generator-specific behavior goes in that generator's module, not in `transform.rs`.

## Definition of done

```bash
cargo test && cargo fmt --check && cargo clippy --all-targets -- -D warnings
```

Every upstream fixture must match the real Babel plugin's output, including the
`*_through_the_consumer_pipeline` tests. A golden is never hand-edited: regenerate it with
`node tools/gen.mjs` and review the diff. If a change makes a golden move, the change is
wrong until proven otherwise — the goldens are Babel's actual output.

## Known coupling

`uid.rs` reproduces `@babel/traverse`'s `generateUid` suffix sequence, which that package has
changed once already (the `_el$0` / `_el$1` detour after `_el$9` is current behavior; older
releases counted straight on to `_el$10`). Generated identifier names are part of the output
contract, so if the consumer's Babel version moves, re-run `tools/verify-goldens.mjs` against
it before assuming the port still matches.

The `dynamic` and `universal` generators are not ported, which leaves 6 of upstream's 11 spec
files out of scope. The 5 that are in scope use every fixture group upstream has.

Run the corpus through **all four** configurations: `CORPUS_GENERATE=ssr` and
`CORPUS_HYDRATABLE=1` combine into dom, ssr, dom+hydratable and ssr+hydratable. `CORPUS_GENERATE=ssr` recompiles it with the SSR
generator, and three bugs hid there that thousands of DOM-only files never touched — the SSR
paths have their own copies of checks like `isBinaryExpression`, so fixing one side does not
fix the other. Never run two mutation passes at once: they save and restore the same source
files and will leave a mutation behind.

Every fix has been checked by mutation: undo it in the source, and at least one test must
fail. Do the same for the next one — a fixture that merely *contains* the shape proves
nothing. Two traps this caught here: the golden harness normalises literals and parens on
both sides, so it is blind to anything at that level (use `print_raw` and assert the output
parses), and a fixture with no import parses as a Script, which takes a different branch from
a Module.

Pure helpers with non-obvious semantics (`uid`, `trim_whitespace`, nesting) keep a
`#[cfg(test)] mod tests` beside them; everything else is covered through the fixtures.
