import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div> | <!#><!/> |  |  | <!#><!/> | `);
const template12 = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$2 = _el$.firstChild,
    _el$7 = _el$2.nextSibling,
    [_el$8, _co$] = _$getNextMarker(_el$7.nextSibling),
    _el$3 = _el$8.nextSibling,
    _el$9 = _el$3.nextSibling,
    [_el$0, _co$2] = _$getNextMarker(_el$9.nextSibling),
    _el$6 = _el$0.nextSibling;
  _$insert(_el$, _$createComponent(Link, {
    children: "comments"
  }), _el$8, _co$);
  _$insert(_el$, _$createComponent(Link, {
    children: "show"
  }), _el$0, _co$2);
  return _el$;
})();
