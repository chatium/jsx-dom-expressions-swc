import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { insert as _$insert } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template29 = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$setAttribute(_el$, "attribute", !!someValue);
  _$insert(_el$, !!someValue);
  return _el$;
})();
