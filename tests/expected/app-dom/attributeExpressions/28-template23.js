import { template as _$template } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template23 = (() => {
  const _el$ = _tmpl$();
  _$insert(_el$, () => "t" in test && "true");
  _$effect(() => _el$.disabled = "t" in test);
  return _el$;
})();
