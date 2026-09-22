import { ssr as _$ssr } from "r-server";
import { createComponent as _$createComponent } from "r-server";
const _tmpl$ = "<div></div>";
const template4 = _$createComponent(Child, {
  get children() {
    return _$ssr(_tmpl$);
  }
});
