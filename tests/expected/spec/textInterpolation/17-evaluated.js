import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<span>Hello `);
const evaluated = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild;
  _$insert(_el$, value + "!", null);
  return _el$;
})();
