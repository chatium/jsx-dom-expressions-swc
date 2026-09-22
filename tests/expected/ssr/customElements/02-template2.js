import { ssr as _$ssr } from "r-server";
import { ssrAttribute as _$ssrAttribute } from "r-server";
import { escape as _$escape } from "r-server";
const _tmpl$ = ["<my-element", "></my-element>"];
const template2 = _$ssr(_tmpl$, _$ssrAttribute("some-attr", _$escape(state.name, true), false) + _$ssrAttribute("notprop", _$escape(state.data, true), false) + _$ssrAttribute("my-attr", _$escape(state.data, true), false));
