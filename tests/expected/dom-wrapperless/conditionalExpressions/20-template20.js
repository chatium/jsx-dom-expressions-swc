import { template as _$template } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template20 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, () => state.dynamic ? _$createComponent(Comp, {}) : _$createComponent(Comp, {}));
  return _el$;
})();
