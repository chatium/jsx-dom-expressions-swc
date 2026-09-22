import { template as _$template } from "r-dom";
import { getOwner as _$getOwner } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<my-element>`, true, false);
const template = (() => {
  const _el$ = _tmpl$();
  _el$.someAttr = name;
  _el$.notprop = data;
  _$setAttribute(_el$, "my-attr", data);
  _el$.someProp = data;
  _el$._$owner = _$getOwner();
  return _el$;
})();
