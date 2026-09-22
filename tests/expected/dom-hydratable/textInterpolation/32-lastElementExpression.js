import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><div></div><!#><!/>`);
const lastElementExpression = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.nextSibling,
    [_el$4, _co$] = _$getNextMarker(_el$3.nextSibling);
  _$insert(_el$, expr, _el$4, _co$);
  return _el$;
})();
