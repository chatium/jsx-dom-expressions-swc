import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { createComponent as _$createComponent } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<html", "><!--#-->", "<!--/--><!--#-->", "<!--/--></html>"];
const templateEmptied = _$ssr(_tmpl$, _$ssrHydrationKey(), _$escape(_$createComponent(Head, {})), _$escape(_$createComponent(Body, {})));
