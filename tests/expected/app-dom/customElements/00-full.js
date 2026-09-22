import { template as _$template } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<my-element>`, true, false),
  _tmpl$2 = /*#__PURE__*/_$template(`<my-element><header slot="head">Title`, true, false),
  _tmpl$3 = /*#__PURE__*/_$template(`<slot name="head">`);
const template = (() => {
  const _el$ = _tmpl$();
  _el$.someAttr = name;
  _el$.notprop = data;
  _$setAttribute(_el$, "my-attr", data);
  _el$.someProp = data;
  return _el$;
})();
const template2 = (() => {
  const _el$2 = _tmpl$();
  _$effect(_p$ => {
    const _v$ = state.name,
      _v$2 = state.data,
      _v$3 = state.data,
      _v$4 = state.data;
    _v$ !== _p$._v$ && (_el$2.someAttr = _p$._v$ = _v$);
    _v$2 !== _p$._v$2 && (_el$2.notprop = _p$._v$2 = _v$2);
    _v$3 !== _p$._v$3 && _$setAttribute(_el$2, "my-attr", _p$._v$3 = _v$3);
    _v$4 !== _p$._v$4 && (_el$2.someProp = _p$._v$4 = _v$4);
    return _p$;
  }, {
    _v$: undefined,
    _v$2: undefined,
    _v$3: undefined,
    _v$4: undefined
  });
  return _el$2;
})();
const template3 = _tmpl$2();
const template4 = _tmpl$3();
