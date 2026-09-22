import { template as _$template } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { getNextElement as _$getNextElement } from "solid-js/web";
import { runHydrationEvents as _$runHydrationEvents } from "solid-js/web";
import { getNextMarker as _$getNextMarker } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<button><!#><!/><!#><!/>: <!#><!/> <!#><!/>`);
// A component built by a factory, with a namespace import, a default import and a re-export:
// the shapes an application file mixes around its JSX.
import { createSignal, createComponent, For } from 'solid-js';
import value, * as shared from './shared/value';
import Widget from './shared/Widget';
export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(value);
  return (() => {
    const _el$ = _$getNextElement(_tmpl$),
      _el$4 = _el$.firstChild,
      [_el$5, _co$] = _$getNextMarker(_el$4.nextSibling),
      _el$6 = _el$5.nextSibling,
      [_el$7, _co$2] = _$getNextMarker(_el$6.nextSibling),
      _el$2 = _el$7.nextSibling,
      _el$8 = _el$2.nextSibling,
      [_el$9, _co$3] = _$getNextMarker(_el$8.nextSibling),
      _el$3 = _el$9.nextSibling,
      _el$0 = _el$3.nextSibling,
      [_el$1, _co$4] = _$getNextMarker(_el$0.nextSibling);
    _el$.$$click = () => setCount(count() + 1);
    _$insert(_el$, _$createComponent(Widget, {}), _el$5, _co$);
    _$insert(_el$, () => ctx.t('Count'), _el$7, _co$2);
    _$insert(_el$, count, _el$9, _co$3);
    _$insert(_el$, () => shared.value, _el$1, _co$4);
    _$runHydrationEvents();
    return _el$;
  })();
});
export { For };
export default Page;
_$delegateEvents(["click"]);
