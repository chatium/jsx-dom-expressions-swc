import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template31 = (() => {
  const _el$ = _tmpl$();
  _$effect(() => getStore.itemProperties.color != null ? _el$.style.setProperty("background-color", getStore.itemProperties.color) : _el$.style.removeProperty("background-color"));
  return _el$;
})();
