import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { runHydrationEvents as _$runHydrationEvents } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
import { spread as _$spread } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div start="Hi">Hi`);
const template26 = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$setAttribute(_el$, "middle", middle);
  _$spread(_el$, spread, false, true);
  _$runHydrationEvents();
  return _el$;
})();
