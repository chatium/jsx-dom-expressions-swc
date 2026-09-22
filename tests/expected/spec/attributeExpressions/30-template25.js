import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><a>`);
const template25 = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild;
  _$insert(_el$, () => props.children, _el$2);
  _$spread(_el$2, _$mergeProps(props, {
    "something": ""
  }), false, false);
  return _el$;
})();
