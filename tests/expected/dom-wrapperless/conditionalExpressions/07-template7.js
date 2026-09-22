import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template7 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, () => state.count > 5 ? state.dynamic ? best : good() : bad);
  return _el$;
})();
