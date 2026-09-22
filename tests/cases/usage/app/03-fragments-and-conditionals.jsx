const view = (state, good, bad) => (
  <>
    <span>{state.ready ? good() : bad}</span>
    {state.ready && <hr />}
    {state.items.map(item => (
      <div>{item}</div>
    ))}
    {/* dropped */}
    text tail
  </>
)
