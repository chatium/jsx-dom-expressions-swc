// A module directive stays the first thing in the file; generated imports go after it.
// This file has an import of its own, so it parses as a Module rather than a Script —
// the two take different branches when the prologue is spliced in.
"use client";

import { ssr as _$ssr } from "solid-js/web";
import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
const _tmpl$ = ["<div", ">", "</div>"];
import { createSignal } from "solid-js";
const withDirective = _$ssr(_tmpl$, _$ssrAttribute("class", _$escape(c(), true), false), _$escape(createSignal));
