import { template as _$template } from "r-dom";
import { delegateEvents as _$delegateEvents } from "r-dom";
import { addEventListener as _$addEventListener } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<button class="a b c">Hi`);
const template17 = (() => {
  const _el$ = _tmpl$();
  _$addEventListener(_el$, "click", increment, true);
  return _el$;
})();
_$delegateEvents(["click"]);
