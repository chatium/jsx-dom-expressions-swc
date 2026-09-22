import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
const templateEmptied = (() => {
  const _el$ = _$getNextElement(),
    _el$2 = _el$.firstChild,
    [_el$3, _co$] = _$getNextMarker(_el$2.nextSibling),
    _el$4 = _el$3.nextSibling,
    [_el$5, _co$2] = _$getNextMarker(_el$4.nextSibling);
  _$insert(_el$, _$createComponent(Head, {}), _el$3, _co$);
  _$insert(_el$, _$createComponent(Body, {}), _el$5, _co$2);
  return _el$;
})();
