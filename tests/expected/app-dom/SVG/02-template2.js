import { template as _$template } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<svg width="400" height="180"><rect rx="20" ry="20" width="150" height="150">`);
const template2 = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild;
  _el$2.style.setProperty("fill", "red");
  _el$2.style.setProperty("stroke", "black");
  _el$2.style.setProperty("opacity", "0.5");
  _$effect(_p$ => {
    const _v$ = state.name,
      _v$2 = state.width,
      _v$3 = state.x,
      _v$4 = state.y,
      _v$5 = props.stroke;
    _v$ !== _p$._v$ && _$setAttribute(_el$2, "class", _p$._v$ = _v$);
    _v$2 !== _p$._v$2 && _$setAttribute(_el$2, "stroke-width", _p$._v$2 = _v$2);
    _v$3 !== _p$._v$3 && _$setAttribute(_el$2, "x", _p$._v$3 = _v$3);
    _v$4 !== _p$._v$4 && _$setAttribute(_el$2, "y", _p$._v$4 = _v$4);
    _v$5 !== _p$._v$5 && ((_p$._v$5 = _v$5) != null ? _el$2.style.setProperty("stroke-width", _v$5) : _el$2.style.removeProperty("stroke-width"));
    return _p$;
  }, {
    _v$: undefined,
    _v$2: undefined,
    _v$3: undefined,
    _v$4: undefined,
    _v$5: undefined
  });
  return _el$;
})();
