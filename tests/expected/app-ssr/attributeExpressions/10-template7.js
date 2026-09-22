import { ssr as _$ssr } from "solid-js/web";
import { ssrStyle as _$ssrStyle } from "solid-js/web";
const _tmpl$ = ["<div style=\"", "\" class=\"", "\"></div>"];
const template7 = _$ssr(_tmpl$, _$ssrStyle({
  "background-color": color(),
  "margin-right": "40px",
  ...props.style,
  "padding-top": props.top
}), props.active ? "my-class" : "");
