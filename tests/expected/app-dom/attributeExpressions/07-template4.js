import { template as _$template } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { classList as _$classList } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template4 = (() => {
  const _el$ = _tmpl$();
  _$classList(_el$, {
    "ccc:ddd": true
  });
  _$effect(() => _$className(_el$, `hi ${state.class || ""}`));
  return _el$;
})();
