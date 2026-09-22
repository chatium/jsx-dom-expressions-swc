import { ssr as _$ssr } from "solid-js/web";
import { ssrHydrationKey as _$ssrHydrationKey } from "solid-js/web";
const _tmpl$ = ["<style", ">.a { color: red }</style>"];
// The same const, declared before its use, does fold into the template.
const EARLIER_STYLES = `.a { color: red }`;
const inOrder = _$ssr(_tmpl$, _$ssrHydrationKey());
