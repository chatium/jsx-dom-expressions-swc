import { template as _$template } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<input type="checkbox">`);
const template14 = (() => {
  const _el$ = _tmpl$();
  _$effect(() => _el$.checked = state.visible);
  return _el$;
})();
