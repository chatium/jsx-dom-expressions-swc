import { template as _$template } from "solid-js/web";
import { getNextElement as _$getNextElement } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<span>`),
  _tmpl$2 = /*#__PURE__*/_$template(`<hr>`),
  _tmpl$3 = /*#__PURE__*/_$template(`<div>`);
const view = (state, good, bad) => [(() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$insert(_el$, (() => {
    const _c$ = _$memo(() => !!state.ready);
    return () => _c$() ? good() : bad;
  })());
  return _el$;
})(), _$memo((() => {
  const _c$2 = _$memo(() => !!state.ready);
  return () => _c$2() && _$getNextElement(_tmpl$2);
})()), _$memo(() => state.items.map(item => (() => {
  const _el$3 = _$getNextElement(_tmpl$3);
  _$insert(_el$3, item);
  return _el$3;
})())), "text tail"];
