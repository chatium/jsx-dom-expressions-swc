export function Card(props) {
  let node
  const rest = () => props.rest
  return (
    <section {...rest()} ref={node} data-id={props.id} aria-label={props.label || 'card'}>
      <h2 innerHTML={props.title} />
      <p>{props.body}</p>
      {props.children}
    </section>
  )
}
