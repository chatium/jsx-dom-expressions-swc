import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<module>Hi <!#><!/>`);
const template18 = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$2 = _el$.firstChild,
    [_el$3, _co$] = _$getNextMarker(_el$2.nextSibling);
  _$insert(_el$, children, _el$3, _co$);
  return _el$;
})();
