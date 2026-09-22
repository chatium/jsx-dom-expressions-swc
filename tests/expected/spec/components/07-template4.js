import { template as _$template } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template4 = _$createComponent(Child, {
  ref(r$) {
    const _ref$ = set;
    typeof _ref$ === "function" ? _ref$(r$) : set = r$;
  },
  get children() {
    return _tmpl$();
  }
});
