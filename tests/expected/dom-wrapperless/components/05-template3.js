import { template as _$template } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template3 = _$createComponent(Child, {
  get children() {
    return [_tmpl$(), _tmpl$(), _tmpl$(), "After"];
  }
});
