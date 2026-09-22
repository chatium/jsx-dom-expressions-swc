import { ssr as _$ssr } from "r-server";
import { ssrElement as _$ssrElement } from "r-server";
import { mergeProps as _$mergeProps } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", "><!--#-->", "<!--/-->", "</div>"];
const template25 = _$ssr(_tmpl$, _$ssrHydrationKey(), _$escape(props.children), _$ssrElement("a", _$mergeProps(props, {
  something: true
}), undefined, false));
