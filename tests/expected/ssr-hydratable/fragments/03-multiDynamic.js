import { ssr as _$ssr } from "r-server";
import { ssrAttribute as _$ssrAttribute } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", ">First</div>"],
  _tmpl$2 = ["<div", ">Last</div>"];
const multiDynamic = [_$ssr(_tmpl$, _$ssrHydrationKey() + _$ssrAttribute("id", _$escape(state.first, true), false)), state.inserted, _$ssr(_tmpl$2, _$ssrHydrationKey() + _$ssrAttribute("id", _$escape(state.last, true), false)), "After"];
