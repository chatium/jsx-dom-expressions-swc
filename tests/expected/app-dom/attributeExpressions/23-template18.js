import { template as _$template } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
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
