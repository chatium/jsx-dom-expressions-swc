import { createComponent as _$createComponent } from "r-server";
import { ssr as _$ssr } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", "></div>"],
  _tmpl$2 = ["<span", "></span>"];
const template3 = _$createComponent(Component, {
  get children() {
    return [_$ssr(_tmpl$, _$ssrHydrationKey()), _$ssr(_tmpl$2, _$ssrHydrationKey())];
  }
});
