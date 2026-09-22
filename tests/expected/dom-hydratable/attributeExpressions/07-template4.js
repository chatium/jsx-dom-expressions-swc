import { template as _$template } from "r-dom";
import { className as _$className } from "r-dom";
import { effect as _$effect } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { classList as _$classList } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template4 = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$classList(_el$, {
    "ccc:ddd": true
  });
  _$effect(() => _$className(_el$, `hi ${state.class || ""}`));
  return _el$;
})();
