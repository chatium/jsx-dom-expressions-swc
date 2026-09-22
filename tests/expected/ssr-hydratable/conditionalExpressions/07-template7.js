import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", ">", "</div>"];
const template7 = _$ssr(_tmpl$, _$ssrHydrationKey(), state.count > 5 ? state.dynamic ? _$escape(best) : _$escape(good()) : _$escape(bad));
