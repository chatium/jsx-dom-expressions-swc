import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
import { spread as _$spread } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><div></div><div> </div><div>`);
const template2 = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.nextSibling,
    _el$4 = _el$3.firstChild,
    _el$5 = _el$3.nextSibling;
  _$spread(_el$, _$mergeProps(() => getProps("test")), false, true);
  _el$2.textContent = rowId;
  _el$5.innerHTML = "<div/>";
  _$effect(() => _el$4.data = row.label);
  return _el$;
})();
