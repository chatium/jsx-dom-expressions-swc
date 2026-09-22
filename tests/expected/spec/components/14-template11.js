import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div> | <!> | <!> | `);
const template11 = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$5 = _el$2.nextSibling,
    _el$3 = _el$5.nextSibling,
    _el$6 = _el$3.nextSibling,
    _el$4 = _el$6.nextSibling;
  _$insert(_el$, _$createComponent(Link, {
    children: "new"
  }), _el$2);
  _$insert(_el$, _$createComponent(Link, {
    children: "comments"
  }), _el$5);
  _$insert(_el$, _$createComponent(Link, {
    children: "show"
  }), _el$5);
  _$insert(_el$, _$createComponent(Link, {
    children: "ask"
  }), _el$6);
  _$insert(_el$, _$createComponent(Link, {
    children: "jobs"
  }), _el$6);
  _$insert(_el$, _$createComponent(Link, {
    children: "submit"
  }), null);
  return _el$;
})();
