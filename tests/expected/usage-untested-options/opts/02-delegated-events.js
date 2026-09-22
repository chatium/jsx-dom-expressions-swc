import { template as _$template } from "r-dom";
import { delegateEvents as _$delegateEvents } from "r-dom";
import { addEventListener as _$addEventListener } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`);
// delegatedEvents adds names to the built-in delegation list; others stay plain listeners.
const events = (() => {
  const _el$ = _tmpl$();
  _$addEventListener(_el$, "other", h);
  _$addEventListener(_el$, "click", h, true);
  _$addEventListener(_el$, "customev", h, true);
  return _el$;
})();
_$delegateEvents(["customev", "click"]);
