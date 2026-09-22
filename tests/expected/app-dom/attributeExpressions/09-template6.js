import { template as _$template } from "solid-js/web";
import { style as _$style } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template6 = (() => {
  const _el$ = _tmpl$();
  _el$.textContent = "Hi";
  _$effect(_$p => _$style(_el$, someStyle(), _$p));
  return _el$;
})();
