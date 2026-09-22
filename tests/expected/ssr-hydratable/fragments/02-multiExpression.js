import { ssr as _$ssr } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", ">First</div>"],
  _tmpl$2 = ["<div", ">Last</div>"];
const multiExpression = [_$ssr(_tmpl$, _$ssrHydrationKey()), inserted, _$ssr(_tmpl$2, _$ssrHydrationKey()), "After"];
