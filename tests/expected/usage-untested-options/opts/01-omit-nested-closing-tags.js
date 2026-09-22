import { template as _$template } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><p>one</p><span>two</span><table><tbody><tr><td>x`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div><a>link</a><b>bold</b>tail`);
// omitNestedClosingTags drops a closing tag whose parent will close it anyway.
const nested = _tmpl$();
const inline = _tmpl$2();
