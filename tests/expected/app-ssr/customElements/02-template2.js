import { ssr as _$ssr } from "solid-js/web";
import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
const _tmpl$ = ["<my-element", "></my-element>"];
const template2 = _$ssr(_tmpl$, _$ssrAttribute("some-attr", _$escape(state.name, true), false) + _$ssrAttribute("notprop", _$escape(state.data, true), false) + _$ssrAttribute("my-attr", _$escape(state.data, true), false));
