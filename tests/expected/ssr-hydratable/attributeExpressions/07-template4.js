import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", " class=\"", "\"></div>"];
const template4 = _$ssr(_tmpl$, _$ssrHydrationKey(), `hi ${_$escape(state.class, true) || ""} ccc:ddd`);
