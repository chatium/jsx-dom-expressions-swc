import { createComponent as _$createComponent } from "r-server";
import { ssr as _$ssr } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", "></div>"];
const template7 = _$createComponent(Child, {
  get children() {
    return [_$ssr(_tmpl$, _$ssrHydrationKey()), state.dynamic];
  }
});
