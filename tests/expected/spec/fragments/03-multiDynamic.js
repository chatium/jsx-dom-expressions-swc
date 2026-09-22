import { template as _$template } from "r-dom";
import { memo as _$memo } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>First`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div>Last`);
const multiDynamic = [(() => {
  const _el$ = _tmpl$();
  _$effect(() => _$setAttribute(_el$, "id", state.first));
  return _el$;
})(), _$memo(() => state.inserted), (() => {
  const _el$2 = _tmpl$2();
  _$effect(() => _$setAttribute(_el$2, "id", state.last));
  return _el$2;
})(), "After"];
