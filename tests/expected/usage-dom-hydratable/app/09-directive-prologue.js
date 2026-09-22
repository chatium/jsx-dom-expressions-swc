// A module directive stays the first thing in the file; generated imports go after it.
// This file has an import of its own, so it parses as a Module rather than a Script —
// the two take different branches when the prologue is spliced in.
"use client";

import { template as _$template } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { getNextElement as _$getNextElement } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
import { createSignal } from "solid-js";
const withDirective = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$insert(_el$, createSignal);
  _$effect(() => _$className(_el$, c()));
  return _el$;
})();
