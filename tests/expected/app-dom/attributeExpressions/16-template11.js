import { template as _$template } from "solid-js/web";
import { use as _$use } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template11 = (() => {
  const _el$ = _tmpl$();
  _$use(zero, _el$, () => 0);
  _$use(another, _el$, () => thing);
  _$use(something, _el$, () => true);
  return _el$;
})();
