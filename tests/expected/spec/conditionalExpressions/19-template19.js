import { template as _$template } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template19 = (() => {
  const _el$ = _tmpl$();
  _$effect(() => _el$.innerHTML = state.dynamic ? _$createComponent(Comp, {}) : _$createComponent(Comp, {}));
  return _el$;
})();
