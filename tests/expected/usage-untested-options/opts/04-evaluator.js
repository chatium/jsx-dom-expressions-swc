import { template as _$template } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div a="3" b="7" c="20" d="3" e="3" f="1024">`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div a="lt" b="le" c="le" d="ge" e="eq" f="ne">`),
  _tmpl$3 = /*#__PURE__*/_$template(`<div a="s" b="eq" c="loose">`),
  _tmpl$4 = /*#__PURE__*/_$template(`<div a="1" b="7" c="6" d="16" e="-4" f="15">`),
  _tmpl$5 = /*#__PURE__*/_$template(`<div a="-5" b="3" c="y" d="string" e="number" f="-6">`),
  _tmpl$6 = /*#__PURE__*/_$template(`<div a="1x" b="2" c="1" d="10" e="false">`),
  _tmpl$7 = /*#__PURE__*/_$template(`<div a="x  " b="  x" c="ab1" d="255">`),
  _tmpl$8 = /*#__PURE__*/_$template(`<div a="7" b="n1">`);
// The evaluator decides what folds into a template, so every operator it implements is
// compared against Babel's own folding here — these were written from the spec, not observed.
const arith = _tmpl$();
const compare = _tmpl$2();
const strings = _tmpl$3();
const bits = _tmpl$4();
const unary = _tmpl$5();
const coercion = _tmpl$6();
const methods = _tmpl$7();
const nested = _tmpl$8();
