import { ssr as _$ssr } from "r-server";
import { ssrAttribute as _$ssrAttribute } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", " foo", " style=\"", "\"", ">", "</div>"];
const template3 = _$ssr(_tmpl$, _$ssrHydrationKey(), _$ssrAttribute("id", _$escape(/*@once*/state.id, true), false), "background-color:" + _$escape(state.color, true), _$ssrAttribute("name", _$escape(state.name, true), false), _$escape(/*@once*/state.content) || " ");
