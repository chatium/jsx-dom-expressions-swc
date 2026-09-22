import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<span><!#><!/> <!#><!/>`);
const multiExpr = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$3 = _el$.firstChild,
    [_el$4, _co$] = _$getNextMarker(_el$3.nextSibling),
    _el$2 = _el$4.nextSibling,
    _el$5 = _el$2.nextSibling,
    [_el$6, _co$2] = _$getNextMarker(_el$5.nextSibling);
  _$insert(_el$, greeting, _el$4, _co$);
  _$insert(_el$, name, _el$6, _co$2);
  return _el$;
})();
