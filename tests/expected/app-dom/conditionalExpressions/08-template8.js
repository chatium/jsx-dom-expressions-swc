import { template as _$template } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template8 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, (() => {
    const _c$ = _$memo(() => !!(state.dynamic && state.something));
    return () => _c$() && good();
  })());
  return _el$;
})();
