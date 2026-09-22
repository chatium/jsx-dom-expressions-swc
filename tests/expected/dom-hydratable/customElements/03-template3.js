import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { getOwner as _$getOwner } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<my-element><header slot="head">Title`, true, false);
const template3 = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _el$._$owner = _$getOwner();
  return _el$;
})();
