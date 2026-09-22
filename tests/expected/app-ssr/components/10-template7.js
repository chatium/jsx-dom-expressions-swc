import { createComponent as _$createComponent } from "solid-js/web";
import { ssr as _$ssr } from "solid-js/web";
const _tmpl$ = "<div></div>";
const template7 = _$createComponent(Child, {
  get children() {
    return [_$ssr(_tmpl$), state.dynamic];
  }
});
