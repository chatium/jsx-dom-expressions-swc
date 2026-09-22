import { template as _$template } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
import { memo as _$memo } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template7 = _$createComponent(Child, {
  get children() {
    return [_tmpl$(), _$memo(() => state.dynamic)];
  }
});
