import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
import { memo as _$memo } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template11 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, (() => {
    const _c$ = _$memo(() => !!state.a);
    return () => _c$() ? a() : (() => {
      const _c$2 = _$memo(() => !!state.b);
      return () => _c$2() ? b() : state.c ? "c" : "fallback";
    })();
  })());
  return _el$;
})();
