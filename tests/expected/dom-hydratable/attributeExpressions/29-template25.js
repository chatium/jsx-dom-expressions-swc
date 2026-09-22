import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { runHydrationEvents as _$runHydrationEvents } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><!#><!/><a>`);
const template25 = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$3 = _el$.firstChild,
    [_el$4, _co$] = _$getNextMarker(_el$3.nextSibling),
    _el$2 = _el$4.nextSibling;
  _$insert(_el$, () => props.children, _el$4, _co$);
  _$spread(_el$2, _$mergeProps(props, {
    "something": ""
  }), false, false);
  _$runHydrationEvents();
  return _el$;
})();
