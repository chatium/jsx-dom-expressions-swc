import { ssr as _$ssr } from "solid-js/web";
import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
const _tmpl$ = ["<div", ">First</div>"],
  _tmpl$2 = ["<div", ">Last</div>"];
const multiDynamic = [_$ssr(_tmpl$, _$ssrAttribute("id", _$escape(state.first, true), false)), state.inserted, _$ssr(_tmpl$2, _$ssrAttribute("id", _$escape(state.last, true), false)), "After"];
