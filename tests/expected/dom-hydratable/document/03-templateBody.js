import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
const templateBody = (() => {
  const _el$ = _$getNextElement(),
    _el$2 = _el$.firstChild,
    _el$4 = _el$2.nextSibling,
    [_el$5, _co$] = _$getNextMarker(_el$4.nextSibling),
    _el$3 = _el$5.nextSibling;
  _$insert(_el$, _$createComponent(App, {}), _el$5, _co$);
  return _el$;
})();
