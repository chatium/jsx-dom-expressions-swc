import { ssr as _$ssr } from "r-server";
import { ssrStyle as _$ssrStyle } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", " style=\"", "\" class=\"", "\"></div>"];
const template7 = _$ssr(_tmpl$, _$ssrHydrationKey(), _$ssrStyle({
  "background-color": color(),
  "margin-right": "40px",
  ...props.style,
  "padding-top": props.top
}), props.active ? "my-class" : "");
