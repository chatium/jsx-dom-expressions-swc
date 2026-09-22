import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
const _tmpl$ = ["<div class=\"", "\"></div>"];
const template4 = _$ssr(_tmpl$, `hi ${_$escape(state.class, true) || ""} ccc:ddd`);
