import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { runHydrationEvents as _$runHydrationEvents } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div start="Hi">Hi`);
const template27 = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$spread(_el$, _$mergeProps(first, {
    "middle": middle
  }, second), false, true);
  _$runHydrationEvents();
  return _el$;
})();
