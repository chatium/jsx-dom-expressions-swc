import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
import { memo as _$memo } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template8 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, (() => {
    const _c$ = _$memo(() => !!(state.dynamic && state.something));
    return () => _c$() && good();
  })());
  return _el$;
})();
