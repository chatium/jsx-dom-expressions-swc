import { template as _$template } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<span> <!> `);
const multiExprTogether = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$4 = _el$2.nextSibling,
    _el$3 = _el$4.nextSibling;
  _$insert(_el$, greeting, _el$4);
  _$insert(_el$, name, _el$4);
  return _el$;
})();
