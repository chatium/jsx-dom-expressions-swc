import { template as _$template } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { use as _$use } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>Hello <!#><!/>`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div>`);
const Child = props => {
  const [s, set] = createSignal();
  return [(() => {
    const _el$ = _$getNextElement(_tmpl$),
      _el$2 = _el$.firstChild,
      _el$3 = _el$2.nextSibling,
      [_el$4, _co$] = _$getNextMarker(_el$3.nextSibling);
    const _ref$ = props.ref;
    typeof _ref$ === "function" ? _$use(_ref$, _el$) : props.ref = _el$;
    _$insert(_el$, () => props.name, _el$4, _co$);
    return _el$;
  })(), (() => {
    const _el$5 = _$getNextElement(_tmpl$2);
    _$use(set, _el$5);
    _$insert(_el$5, () => props.children);
    return _el$5;
  })()];
};
