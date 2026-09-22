import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<input type="checkbox">`);
const template14 = (() => {
  const _el$ = _tmpl$();
  _$effect(() => _el$.checked = state.visible);
  return _el$;
})();
