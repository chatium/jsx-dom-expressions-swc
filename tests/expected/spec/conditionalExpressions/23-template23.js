import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template23 = (() => {
  const _el$ = _tmpl$();
  _$effect(() => _el$.innerHTML = state?.dynamic ? "a" : "b");
  return _el$;
})();
