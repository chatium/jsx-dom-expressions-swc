import { ssrElement as _$ssrElement } from "r-server";
import { mergeProps as _$mergeProps } from "r-server";
import { ssr as _$ssr } from "r-server";
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
}), () => _$ssrElement("h1", _$mergeProps({
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
}), () => _$ssr(_tmpl$, "ccc ddd"), false), true);
