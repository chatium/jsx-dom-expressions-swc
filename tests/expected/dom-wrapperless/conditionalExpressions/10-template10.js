import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template10 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, () => state.a ? "a" : state.b ? "b" : state.c ? "c" : "fallback");
  return _el$;
})();
