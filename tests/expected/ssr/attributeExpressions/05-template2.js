import { ssrElement as _$ssrElement } from "r-server";
import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
const _tmpl$ = ["<div>", "</div>"],
  _tmpl$2 = "<div><div/></div>";
const template2 = _$ssrElement("div", getProps("test"), [_$ssr(_tmpl$, _$escape(rowId)), _$ssr(_tmpl$, _$escape(row.label)), _$ssr(_tmpl$2)], false);
