import { template as _$template } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template3 = _$createComponent(Child, {
  get children() {
    return [_tmpl$(), _tmpl$(), _tmpl$(), "After"];
  }
});
