// A component built by a factory, with a namespace import, a default import and a re-export:
// the shapes an application file mixes around its JSX.
import { createSignal, createComponent, For } from 'solid-js'
import value, * as shared from './shared/value'
import Widget from './shared/Widget'

export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(value)
  return (
    <button onClick={() => setCount(count() + 1)}>
      <Widget />
      {ctx.t('Count')}: {count()} {shared.value}
    </button>
  )
})

export { For }
export default Page
