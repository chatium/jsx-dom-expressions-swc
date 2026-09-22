import { template as _$template } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
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
