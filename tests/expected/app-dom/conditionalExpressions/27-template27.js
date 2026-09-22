import { template as _$template } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template27 = (() => {
  const _el$ = _tmpl$();
  _$effect(() => _el$.innerHTML = state.dynamic ?? _$createComponent(Comp, {}));
  return _el$;
})();
