import { template as _$template } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template20 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, (() => {
    const _c$ = _$memo(() => !!state.dynamic);
    return () => _c$() ? _$createComponent(Comp, {}) : _$createComponent(Comp, {});
  })());
  return _el$;
})();
