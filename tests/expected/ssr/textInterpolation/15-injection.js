import { ssr as _$ssr } from "r-server";
const _tmpl$ = "<span>Hi&lt;script>alert();&lt;/script></span>";
const injection = _$ssr(_tmpl$);
