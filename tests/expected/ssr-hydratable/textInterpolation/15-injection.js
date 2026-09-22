import { ssr as _$ssr } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<span", ">Hi&lt;script>alert();&lt;/script></span>"];
const injection = _$ssr(_tmpl$, _$ssrHydrationKey());
