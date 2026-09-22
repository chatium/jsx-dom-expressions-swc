// The shape an application block takes after TypeScript is stripped.
import { createSignal, createComponent, For, Show } from 'solid-js'
import Widget from './shared/Widget'

export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(0)
  const [items, setItems] = createSignal([])
  return (
    <div class="page" classList={{ empty: items().length === 0 }}>
      <button onClick={() => setCount(count() + 1)}>
        <Widget />
        {ctx.t('Count')}: {count()}
      </button>
      <Show when={count() > 0} fallback={<em>nothing yet</em>}>
        <ul>
          <For each={items()}>{item => <li title={item.name}>{item.name}</li>}</For>
        </ul>
      </Show>
      <input
        type="text"
        value={count()}
        style={{ color: count() > 5 ? 'red' : 'inherit', 'margin-top': '4px' }}
        onInput={e => setItems([...items(), { name: e.target.value }])}
      />
    </div>
  )
})
