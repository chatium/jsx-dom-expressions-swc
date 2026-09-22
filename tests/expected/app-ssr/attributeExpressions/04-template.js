import { ssrElement as _$ssrElement } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { ssr as _$ssr } from "solid-js/web";
const _tmpl$ = ["<a href=\"/\" class=\"", "\">Welcome</a>"];
const template = _$ssrElement("div", _$mergeProps({
  id: "main"
}, results, {
  classList: {
    selected: unknown
  },
  style: {
    color
  }
}), _$ssrElement("h1", _$mergeProps({
  get ["class"]() {
    return `base ${dynamic() ? "dynamic" : ""} ${selected ? "selected" : ""}`;
  },
  id: id
}, results, {
  foo: true,
  disabled: true,
  get title() {
    return welcoming();
  },
  get style() {
    return {
      "background-color": color(),
      "margin-right": "40px"
    };
  }
}), _$ssr(_tmpl$, "ccc ddd"), false), false);
