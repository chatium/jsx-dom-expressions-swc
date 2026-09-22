import { template as _$template } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template19 = (() => {
  const _el$ = _tmpl$();
  _$effect(() => _el$.innerHTML = state.dynamic ? _$createComponent(Comp, {}) : _$createComponent(Comp, {}));
  return _el$;
})();
