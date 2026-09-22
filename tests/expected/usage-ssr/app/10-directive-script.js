// The same, with no import: this one parses as a Script and becomes a Module.
"use client";

import { ssr as _$ssr } from "solid-js/web";
import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
const _tmpl$ = ["<div", ">hi</div>"];
const scriptWithDirective = _$ssr(_tmpl$, _$ssrAttribute("class", _$escape(c(), true), false));
