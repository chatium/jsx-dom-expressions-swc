import { template as _$template } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div> | <!> |  |  | <!> | `);
const template12 = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$7 = _el$2.nextSibling,
    _el$3 = _el$7.nextSibling,
    _el$8 = _el$3.nextSibling,
    _el$6 = _el$8.nextSibling;
  _$insert(_el$, _$createComponent(Link, {
    children: "comments"
  }), _el$7);
  _$insert(_el$, _$createComponent(Link, {
    children: "show"
  }), _el$8);
  return _el$;
})();
