import { ssr as _$ssr } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", ">First</div>"],
  _tmpl$2 = ["<div", ">Last</div>"];
const multiStatic = [_$ssr(_tmpl$, _$ssrHydrationKey()), _$ssr(_tmpl$2, _$ssrHydrationKey())];
