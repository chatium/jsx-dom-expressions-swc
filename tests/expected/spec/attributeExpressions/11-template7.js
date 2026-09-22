import { template as _$template } from "r-dom";
import { style as _$style } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
const template7 = (() => {
  const _el$ = _tmpl$();
  _el$.classList.toggle("other-class", !!undefVar);
  _el$.classList.toggle("other-class2", !!undefVar);
  _$effect(_p$ => {
    const _v$ = {
        "background-color": color(),
        "margin-right": "40px",
        ...props.style
      },
      _v$2 = props.top,
      _v$3 = !!props.active;
    _p$._v$ = _$style(_el$, _v$, _p$._v$);
    _v$2 !== _p$._v$2 && ((_p$._v$2 = _v$2) != null ? _el$.style.setProperty("padding-top", _v$2) : _el$.style.removeProperty("padding-top"));
    _v$3 !== _p$._v$3 && _el$.classList.toggle("my-class", _p$._v$3 = _v$3);
    return _p$;
  }, {
    _v$: undefined,
    _v$2: undefined,
    _v$3: undefined
  });
  return _el$;
})();
