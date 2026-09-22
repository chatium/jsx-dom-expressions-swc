// The same, with no import: this one parses as a Script and becomes a Module.
"use client";

import { template as _$template } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>hi`);
const scriptWithDirective = (() => {
  const _el$ = _tmpl$();
  _$effect(() => _$className(_el$, c()));
  return _el$;
})();
