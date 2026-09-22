import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { runHydrationEvents as _$runHydrationEvents } from "r-dom";
import { innerHTML as _$innerHTML } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><div></div><div> </div><div>`);
const template2 = (() => {
  const _el$ = _$getNextElement(_tmpl$),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.nextSibling,
    _el$4 = _el$3.firstChild,
    _el$5 = _el$3.nextSibling;
  _$spread(_el$, _$mergeProps(() => getProps("test")), false, true);
  _el$2.textContent = rowId;
  _$innerHTML(_el$5, "<div/>");
  _$effect(() => _el$4.data = row.label);
  _$runHydrationEvents();
  return _el$;
})();
