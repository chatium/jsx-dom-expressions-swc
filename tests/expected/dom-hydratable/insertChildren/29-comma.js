import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const comma = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$insert(_el$, () => (expression(), "static"));
  return _el$;
})();
