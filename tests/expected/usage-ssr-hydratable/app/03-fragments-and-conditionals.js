import { ssr as _$ssr } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
import { ssrHydrationKey as _$ssrHydrationKey } from "solid-js/web";
const _tmpl$ = ["<span", ">", "</span>"],
  _tmpl$2 = ["<hr", ">"],
  _tmpl$3 = ["<div", ">", "</div>"];
const view = (state, good, bad) => [_$ssr(_tmpl$, _$ssrHydrationKey(), state.ready ? _$escape(good()) : _$escape(bad)), state.ready && _$ssr(_tmpl$2, _$ssrHydrationKey()), state.items.map(item => _$ssr(_tmpl$3, _$ssrHydrationKey(), _$escape(item))), "text tail"];
