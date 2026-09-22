// Regenerates tests/expected/<config>/… by running the real babel-plugin-jsx-dom-expressions.
// The goldens are what we must match; never hand-edit them.
import fs from 'node:fs'
import path from 'node:path'
import babel from '@babel/core'
import solid from 'babel-plugin-jsx-dom-expressions'

const root = path.join(import.meta.dirname, '..')
const configs = JSON.parse(fs.readFileSync(path.join(import.meta.dirname, 'configs.json'), 'utf8'))

// Drop goldens for configurations that no longer exist, so a rename cannot leave them behind.
const expected = path.join(root, 'tests', 'expected')
if (fs.existsSync(expected)) {
  for (const dir of fs.readdirSync(expected)) {
    if (!(dir in configs)) fs.rmSync(path.join(expected, dir), { recursive: true, force: true })
  }
}

for (const [name, { fixtures, options }] of Object.entries(configs)) {
  const cases = path.join(root, 'tests', 'cases', fixtures)
  const out = path.join(root, 'tests', 'expected', name)
  fs.rmSync(out, { recursive: true, force: true })
  for (const group of fs.readdirSync(cases).sort()) {
    fs.mkdirSync(path.join(out, group), { recursive: true })
    for (const file of fs.readdirSync(path.join(cases, group)).sort()) {
      const source = fs.readFileSync(path.join(cases, group, file), 'utf8')
      let code
      try {
        code = babel.transformSync(source, {
          filename: path.join(group, file),
          configFile: false,
          babelrc: false,
          highlightCode: false,
          plugins: [[solid, options]],
        }).code
      } catch (error) {
        // A golden the Rust side can never match is a broken fixture, not a test case.
        // Fail here rather than freezing the failure into tests/expected/.
        throw new Error(
          `${name}/${group}/${file}: the Babel plugin rejected this case — fix the fixture.\n` +
            error.message.split('\n')[0],
        )
      }
      fs.writeFileSync(path.join(out, group, file.replace(/\.jsx$/, '.js')), code + '\n')
    }
  }
}
console.log('goldens written')
