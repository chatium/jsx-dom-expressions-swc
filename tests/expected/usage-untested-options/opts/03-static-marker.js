import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
import { insert as _$insert } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
// staticMarker names the comment that freezes an expression; here it is not "@once".
const marked = (() => {
  const _el$ = _tmpl$();
  _$setAttribute(_el$, "title", x.y);
  _$insert(_el$, f());
  return _el$;
})();

// A marker that does not match stays dynamic. Only the attribute form is pinned here: in the
// child form Babel unwraps `f()` to `f` and loses the comment with it, while SWC keys comments
// by position — `f()` and `f` start at the same byte, so it survives. Same AST either way.
const unmarked = (() => {
  const _el$2 = _tmpl$();
  _$insert(_el$2, f);
  _$effect(() => _$setAttribute(_el$2, "title", /*@once*/x.y));
  return _el$2;
})();
