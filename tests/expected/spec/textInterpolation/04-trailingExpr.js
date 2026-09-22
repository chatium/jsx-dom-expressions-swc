import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<span>Hello `);
const trailingExpr = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild;
  _$insert(_el$, name, null);
  return _el$;
})();
