import { template as _$template } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<my-element>`, true, false);
const template2 = (() => {
  const _el$ = _tmpl$();
  _$effect(_p$ => {
    const _v$ = state.name,
      _v$2 = state.data,
      _v$3 = state.data,
      _v$4 = state.data;
    _v$ !== _p$._v$ && (_el$.someAttr = _p$._v$ = _v$);
    _v$2 !== _p$._v$2 && (_el$.notprop = _p$._v$2 = _v$2);
    _v$3 !== _p$._v$3 && _$setAttribute(_el$, "my-attr", _p$._v$3 = _v$3);
    _v$4 !== _p$._v$4 && (_el$.someProp = _p$._v$4 = _v$4);
    return _p$;
  }, {
    _v$: undefined,
    _v$2: undefined,
    _v$3: undefined,
    _v$4: undefined
  });
  return _el$;
})();
