import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { runHydrationEvents as _$runHydrationEvents } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { memo as _$memo } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<label><span>Input is <!#><!/></span><input><div>`);
const template28 = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.firstChild,
    _el$4 = _el$3.nextSibling,
    [_el$5, _co$] = _$getNextMarker(_el$4.nextSibling),
    _el$6 = _el$2.nextSibling,
    _el$7 = _el$6.nextSibling;
  _$spread(_el$, _$mergeProps(api), false, true);
  _$spread(_el$2, _$mergeProps(api), false, true);
  _$insert(_el$2, () => api() ? "checked" : "unchecked", _el$5, _co$);
  _$spread(_el$6, _$mergeProps(api), false, false);
  _$spread(_el$7, _$mergeProps(api), false, false);
  _$runHydrationEvents();
  return _el$;
})();
