import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { runHydrationEvents as _$runHydrationEvents } from "r-dom";
import { insert as _$insert } from "r-dom";
import { spread as _$spread } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<module>`);
const template9 = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$spread(_el$, dynamic, false, true);
  _$insert(_el$, () => dynamic.children);
  _$runHydrationEvents();
  return _el$;
})();
