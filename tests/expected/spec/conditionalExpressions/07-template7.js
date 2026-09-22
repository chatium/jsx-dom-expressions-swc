import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
import { memo as _$memo } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template7 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, (() => {
    const _c$ = _$memo(() => state.count > 5);
    return () => _c$() ? (() => {
      const _c$2 = _$memo(() => !!state.dynamic);
      return () => _c$2() ? best : good();
    })() : bad;
  })());
  return _el$;
})();
