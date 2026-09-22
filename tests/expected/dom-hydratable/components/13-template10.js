import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><!#><!/> | <!#><!/> | <!#><!/> | <!#><!/> | <!#><!/> | <!#><!/>`);
const template10 = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$7 = _el$.firstChild,
    [_el$8, _co$] = _$getNextMarker(_el$7.nextSibling),
    _el$2 = _el$8.nextSibling,
    _el$9 = _el$2.nextSibling,
    [_el$0, _co$2] = _$getNextMarker(_el$9.nextSibling),
    _el$3 = _el$0.nextSibling,
    _el$1 = _el$3.nextSibling,
    [_el$10, _co$3] = _$getNextMarker(_el$1.nextSibling),
    _el$4 = _el$10.nextSibling,
    _el$11 = _el$4.nextSibling,
    [_el$12, _co$4] = _$getNextMarker(_el$11.nextSibling),
    _el$5 = _el$12.nextSibling,
    _el$13 = _el$5.nextSibling,
    [_el$14, _co$5] = _$getNextMarker(_el$13.nextSibling),
    _el$6 = _el$14.nextSibling,
    _el$15 = _el$6.nextSibling,
    [_el$16, _co$6] = _$getNextMarker(_el$15.nextSibling);
  _$insert(_el$, _$createComponent(Link, {
    children: "new"
  }), _el$8, _co$);
  _$insert(_el$, _$createComponent(Link, {
    children: "comments"
  }), _el$0, _co$2);
  _$insert(_el$, _$createComponent(Link, {
    children: "show"
  }), _el$10, _co$3);
  _$insert(_el$, _$createComponent(Link, {
    children: "ask"
  }), _el$12, _co$4);
  _$insert(_el$, _$createComponent(Link, {
    children: "jobs"
  }), _el$14, _co$5);
  _$insert(_el$, _$createComponent(Link, {
    children: "submit"
  }), _el$16, _co$6);
  return _el$;
})();
