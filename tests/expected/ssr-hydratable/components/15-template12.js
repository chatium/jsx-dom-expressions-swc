import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { createComponent as _$createComponent } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", "> | <!--#-->", "<!--/--> |  |  | <!--#-->", "<!--/--> | </div>"];
const template12 = _$ssr(_tmpl$, _$ssrHydrationKey(), _$escape(_$createComponent(Link, {
  children: "comments"
})), _$escape(_$createComponent(Link, {
  children: "show"
})));
