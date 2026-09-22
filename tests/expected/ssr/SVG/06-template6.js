import { createComponent as _$createComponent } from "r-server";
import { ssr as _$ssr } from "r-server";
const _tmpl$ = "<rect x=\"50\" y=\"20\" width=\"150\" height=\"150\"></rect>";
const template6 = _$createComponent(Component, {
  get children() {
    return _$ssr(_tmpl$);
  }
});
