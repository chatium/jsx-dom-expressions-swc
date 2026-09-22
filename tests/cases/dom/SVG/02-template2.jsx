const template2 = (
  <svg width="400" height="180">
    <rect
      className={state.name}
      stroke-width={state.width}
      x={state.x}
      y={state.y}
      rx="20"
      ry="20"
      width="150"
      height="150"
      style={{
        fill: "red",
        stroke: "black",
        "stroke-width": props.stroke,
        opacity: 0.5
      }}
    />
  </svg>
);
