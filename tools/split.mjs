// One-shot: split the upstream fixture suites into one file per top-level statement, plus the
// whole file, which is the only form that reproduces cross-statement scope.
// Usage: node tools/split.mjs <dom-expressions checkout at the pinned commit>
import fs from 'node:fs'
import path from 'node:path'
import { parse } from '@babel/core'

const SUITES = {
  dom: '__dom_fixtures__',
  'dom-hydratable': '__dom_hydratable_fixtures__',
  'dom-wrapperless': '__dom_wrapperless_fixtures__',
  ssr: '__ssr_fixtures__',
  'ssr-hydratable': '__ssr_hydratable_fixtures__',
}

const test = path.join(process.argv[2], 'packages/babel-plugin-jsx-dom-expressions/test')
const out = path.join(import.meta.dirname, '..', 'tests', 'cases')
// Only the upstream suites are regenerated; hand-written ones live alongside them.
for (const suite of Object.keys(SUITES)) {
  fs.rmSync(path.join(out, suite), { recursive: true, force: true })
}

for (const [suite, dir] of Object.entries(SUITES)) {
  const src = path.join(test, dir)
  for (const group of fs.readdirSync(src).sort()) {
    const file = path.join(src, group, 'code.js')
    if (!fs.existsSync(file)) continue
    const code = fs.readFileSync(file, 'utf8')
    const ast = parse(code, {
      filename: file,
      configFile: false,
      babelrc: false,
      plugins: ['@babel/plugin-syntax-jsx'],
    })
    const dest = path.join(out, suite, group)
    fs.mkdirSync(dest, { recursive: true })
    fs.writeFileSync(path.join(dest, '00-full.jsx'), code)
    ast.program.body.forEach((node, i) => {
      const name =
        node.declarations?.[0]?.id?.name ??
        node.id?.name ??
        node.declaration?.declarations?.[0]?.id?.name ??
        `stmt${i + 1}`
      const slug = String(i + 1).padStart(2, '0') + '-' + name.replace(/[^a-zA-Z0-9$_]/g, '_')
      fs.writeFileSync(path.join(dest, slug + '.jsx'), code.slice(node.start, node.end) + '\n')
    })
  }
}
console.log('cases written to', out)
