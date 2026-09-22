import { ssr as _$ssr } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<span", ">1</span>"],
  _tmpl$2 = ["<span", ">2</span>"],
  _tmpl$3 = ["<span", ">3</span>"];
const multiLineTrailing = [_$ssr(_tmpl$, _$ssrHydrationKey()), _$ssr(_tmpl$2, _$ssrHydrationKey()), _$ssr(_tmpl$3, _$ssrHydrationKey())];
