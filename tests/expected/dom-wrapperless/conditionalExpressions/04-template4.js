import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template4 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, () => simple ? good() : bad);
  return _el$;
})();
