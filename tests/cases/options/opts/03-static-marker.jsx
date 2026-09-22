// staticMarker names the comment that freezes an expression; here it is not "@once".
const marked = <div title={/*@static*/ x.y}>{/*@static*/ f()}</div>

// A marker that does not match stays dynamic. Only the attribute form is pinned here: in the
// child form Babel unwraps `f()` to `f` and loses the comment with it, while SWC keys comments
// by position — `f()` and `f` start at the same byte, so it survives. Same AST either way.
const unmarked = <div title={/*@once*/ x.y}>{f()}</div>
