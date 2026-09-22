import { ssr as _$ssr } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = "<div></div>";
const template4 = _$createComponent(Child, {
  get children() {
    return _$ssr(_tmpl$);
  }
});
