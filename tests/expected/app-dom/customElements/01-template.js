import { template as _$template } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<my-element>`, true, false);
const template = (() => {
  const _el$ = _tmpl$();
  _el$.someAttr = name;
  _el$.notprop = data;
  _$setAttribute(_el$, "my-attr", data);
  _el$.someProp = data;
  return _el$;
})();
