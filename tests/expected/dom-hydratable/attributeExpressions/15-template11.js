import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { use as _$use } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template11 = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$use(zero, _el$, () => 0);
  _$use(another, _el$, () => thing);
  _$use(something, _el$, () => true);
  return _el$;
})();
