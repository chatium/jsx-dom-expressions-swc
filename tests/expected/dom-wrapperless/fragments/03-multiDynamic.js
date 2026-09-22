import { template as _$template } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>First`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div>Last`);
const multiDynamic = [(() => {
  const _el$ = _tmpl$();
  _$setAttribute(_el$, "id", state.first);
  return _el$;
})(), () => state.inserted, (() => {
  const _el$2 = _tmpl$2();
  _$setAttribute(_el$2, "id", state.last);
  return _el$2;
})(), "After"];
