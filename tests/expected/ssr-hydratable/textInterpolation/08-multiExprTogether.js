import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<span", "> <!--#-->", "<!--/--><!--#-->", "<!--/--> </span>"];
const multiExprTogether = _$ssr(_tmpl$, _$ssrHydrationKey(), _$escape(greeting), _$escape(name));
