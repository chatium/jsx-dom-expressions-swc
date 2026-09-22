import { ssr as _$ssr } from "solid-js/web";
import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
const _tmpl$ = ["<div foo", " style=\"", "\"", ">", "</div>"];
const template3 = _$ssr(_tmpl$, _$ssrAttribute("id", _$escape(/*@once*/state.id, true), false), "background-color:" + _$escape(state.color, true), _$ssrAttribute("name", _$escape(state.name, true), false), _$escape(state.content));
