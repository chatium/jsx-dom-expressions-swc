import { template as _$template } from "r-dom";
import { style as _$style } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template6 = (() => {
  const _el$ = _tmpl$();
  _el$.textContent = "Hi";
  _$effect(_$p => _$style(_el$, someStyle(), _$p));
  return _el$;
})();
