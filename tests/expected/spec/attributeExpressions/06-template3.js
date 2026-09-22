import { template as _$template } from "r-dom";
import { effect as _$effect } from "r-dom";
import { setAttribute as _$setAttribute } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div foo>`);
const template3 = (() => {
  const _el$ = _tmpl$();
  _$setAttribute(_el$, "id", state.id);
  state.color != null ? _el$.style.setProperty("background-color", state.color) : _el$.style.removeProperty("background-color");
  _el$.textContent = state.content;
  _$effect(() => _$setAttribute(_el$, "name", state.name));
  return _el$;
})();
