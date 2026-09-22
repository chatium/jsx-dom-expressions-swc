import { ssr as _$ssr } from "r-server";
import { ssrAttribute as _$ssrAttribute } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<my-element", "></my-element>"];
const template = _$ssr(_tmpl$, _$ssrHydrationKey() + _$ssrAttribute("some-attr", _$escape(name, true), false) + _$ssrAttribute("notprop", _$escape(data, true), false) + _$ssrAttribute("my-attr", _$escape(data, true), false));
