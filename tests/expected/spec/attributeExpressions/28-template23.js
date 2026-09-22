import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
import { insert as _$insert } from "r-dom";
import { memo as _$memo } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template23 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, () => "t" in test && "true");
  _$effect(() => _el$.disabled = "t" in test);
  return _el$;
})();
