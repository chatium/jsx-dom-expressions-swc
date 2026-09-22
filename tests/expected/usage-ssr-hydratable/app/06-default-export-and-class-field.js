import { ssr as _$ssr } from "solid-js/web";
import { ssrHydrationKey as _$ssrHydrationKey } from "solid-js/web";
const _tmpl$ = ["<span", ">Widget</span>"],
  _tmpl$2 = ["<div", "></div>"];
// A default-exported function component, and JSX as a class field.
export default function Widget() {
  return _$ssr(_tmpl$, _$ssrHydrationKey());
}
export class Example {
  value = _$ssr(_tmpl$2, _$ssrHydrationKey());
}
