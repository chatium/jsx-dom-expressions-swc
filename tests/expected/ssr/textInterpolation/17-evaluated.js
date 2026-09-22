import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
const _tmpl$ = ["<span>Hello ", "</span>"];
const evaluated = _$ssr(_tmpl$, _$escape(value) + "!");
