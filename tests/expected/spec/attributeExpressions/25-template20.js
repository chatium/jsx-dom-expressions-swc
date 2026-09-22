import { template as _$template } from "r-dom";
import { delegateEvents as _$delegateEvents } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
import { addEventListener as _$addEventListener } from "r-dom";
import { effect as _$effect } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div><input readonly=""><input>`);
const template20 = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.nextSibling;
  _$addEventListener(_el$2, "input", doSomething, true);
  _$addEventListener(_el$3, "input", doSomethingElse, true);
  _el$3.readOnly = value;
  _$effect(_p$ => {
    const _v$ = min(),
      _v$2 = max(),
      _v$3 = min(),
      _v$4 = max();
    _v$ !== _p$._v$ && _$setAttribute(_el$2, "min", _p$._v$ = _v$);
    _v$2 !== _p$._v$2 && _$setAttribute(_el$2, "max", _p$._v$2 = _v$2);
    _v$3 !== _p$._v$3 && _$setAttribute(_el$3, "min", _p$._v$3 = _v$3);
    _v$4 !== _p$._v$4 && _$setAttribute(_el$3, "max", _p$._v$4 = _v$4);
    return _p$;
  }, {
    _v$: undefined,
    _v$2: undefined,
    _v$3: undefined,
    _v$4: undefined
  });
  _$effect(() => _el$2.value = s());
  _$effect(() => _el$3.checked = s2());
  return _el$;
})();
_$delegateEvents(["input"]);
