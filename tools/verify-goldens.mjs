// Re-derives every golden with a *different* checkout of Babel and the plugin — normally the
// ones the host toolchain actually ships — and fails on any byte difference.
//
// This is what "compatible" means here: not that we match upstream's committed snapshots
// (those were taken with an older @babel/traverse and no longer reproduce), but that we match
// what the host's own toolchain produces today.
//
// Usage: node tools/verify-goldens.mjs [dir containing node_modules with @babel/core + the plugin]
import fs from 'node:fs'
import path from 'node:path'
import { createRequire } from 'node:module'

const from = process.argv[2] ?? import.meta.dirname
const require = createRequire(path.join(path.resolve(from), 'noop.cjs'))
const babel = require('@babel/core')
const solid = require('babel-plugin-jsx-dom-expressions')

const root = path.join(import.meta.dirname, '..')
const configs = JSON.parse(fs.readFileSync(path.join(import.meta.dirname, 'configs.json'), 'utf8'))

let total = 0
const mismatches = []
for (const [name, { fixtures, options }] of Object.entries(configs)) {
  const cases = path.join(root, 'tests/cases', fixtures)
  for (const group of fs.readdirSync(cases).sort()) {
    for (const file of fs.readdirSync(path.join(cases, group)).sort()) {
      total++
      const out =
        babel.transformSync(fs.readFileSync(path.join(cases, group, file), 'utf8'), {
          filename: path.join(group, file),
          configFile: false,
          babelrc: false,
          highlightCode: false,
          plugins: [[solid, options]],
        }).code + '\n'
      const golden = path.join(root, 'tests/expected', name, group, file.replace(/\.jsx$/, '.js'))
      if (out !== fs.readFileSync(golden, 'utf8')) mismatches.push(`${name}/${group}/${file}`)
    }
  }
}

console.log(
  `${total} goldens re-derived with Babel ${babel.version} from ${path.resolve(from)}; ` +
    `mismatches: ${mismatches.length}`,
)
if (mismatches.length) {
  console.error(mismatches.slice(0, 20).join('\n'))
  process.exitCode = 1
}
