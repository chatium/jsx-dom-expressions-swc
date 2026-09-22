import { ssr as _$ssr } from "r-server";
import { ssrAttribute as _$ssrAttribute } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", "><input", " readonly=\"\"><input", "", "></div>"];
const template20 = _$ssr(_tmpl$, _$ssrHydrationKey(), _$ssrAttribute("value", _$escape(s(), true), false) + _$ssrAttribute("min", _$escape(min(), true), false) + _$ssrAttribute("max", _$escape(max(), true), false), _$ssrAttribute("checked", s2(), true) + _$ssrAttribute("min", _$escape(min(), true), false) + _$ssrAttribute("max", _$escape(max(), true), false), _$ssrAttribute("readonly", value, true));
