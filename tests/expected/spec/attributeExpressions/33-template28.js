import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
import { memo as _$memo } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<label><span>Input is </span><input><div>`);
const template28 = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.firstChild,
    _el$4 = _el$2.nextSibling,
    _el$5 = _el$4.nextSibling;
  _$spread(_el$, _$mergeProps(api), false, true);
  _$spread(_el$2, _$mergeProps(api), false, true);
  _$insert(_el$2, () => api() ? "checked" : "unchecked", null);
  _$spread(_el$4, _$mergeProps(api), false, false);
  _$spread(_el$5, _$mergeProps(api), false, false);
  return _el$;
})();
