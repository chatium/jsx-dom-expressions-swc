import { template as _$template } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<span>Hello `);
const trailingExpr = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild;
  _$insert(_el$, name, null);
  return _el$;
})();
