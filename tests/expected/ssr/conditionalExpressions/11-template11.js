import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
const _tmpl$ = ["<div>", "</div>"];
const template11 = _$ssr(_tmpl$, state.a ? _$escape(a()) : state.b ? _$escape(b()) : state.c ? "c" : "fallback");
