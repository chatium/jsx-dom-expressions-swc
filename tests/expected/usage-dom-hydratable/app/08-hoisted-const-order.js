import { template as _$template } from "solid-js/web";
import { getNextElement as _$getNextElement } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<style>.a { color: red }`);
// The same const, declared before its use, does fold into the template.
const EARLIER_STYLES = `.a { color: red }`;
const inOrder = _$getNextElement(_tmpl$);
