import { ssr as _$ssr } from "solid-js/web";
import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
const _tmpl$ = ["<svg width=\"120\" height=\"40\" viewBox=\"0 0 120 40\"><title>Progress&hellip;</title><rect x=\"0\" y=\"0\"", " height=\"40\" fill=\"currentColor\"></rect><text xlink:href=\"#label\">a &lt; b &amp; c</text></svg>"];
const chart = _$ssr(_tmpl$, _$ssrAttribute("width", _$escape(width(), true), false));
