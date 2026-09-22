import { template as _$template } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template18 = (() => {
  const _el$ = _tmpl$();
  _$spread(_el$, _$mergeProps(() => ({
    get [key()]() {
      return props.value;
    }
  })), false, false);
  return _el$;
})();
