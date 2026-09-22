import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<span> <!> <!> `);
const multiExprSpaced = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$5 = _el$2.nextSibling,
    _el$3 = _el$5.nextSibling,
    _el$6 = _el$3.nextSibling,
    _el$4 = _el$6.nextSibling;
  _$insert(_el$, greeting, _el$5);
  _$insert(_el$, name, _el$6);
  return _el$;
})();
