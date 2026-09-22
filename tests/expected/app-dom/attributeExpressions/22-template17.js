import { template as _$template } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { addEventListener as _$addEventListener } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<button class="a b c">Hi`);
const template17 = (() => {
  const _el$ = _tmpl$();
  _$addEventListener(_el$, "click", increment, true);
  return _el$;
})();
_$delegateEvents(["click"]);
