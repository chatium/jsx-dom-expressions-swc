import { template as _$template } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<button>: <!> `);
// A component built by a factory, with a namespace import, a default import and a re-export:
// the shapes an application file mixes around its JSX.
import { createSignal, createComponent, For } from 'solid-js';
import value, * as shared from './shared/value';
import Widget from './shared/Widget';
export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(value);
  return (() => {
    const _el$ = _tmpl$(),
      _el$2 = _el$.firstChild,
      _el$4 = _el$2.nextSibling,
      _el$3 = _el$4.nextSibling;
    _el$.$$click = () => setCount(count() + 1);
    _$insert(_el$, _$createComponent(Widget, {}), _el$2);
    _$insert(_el$, () => ctx.t('Count'), _el$2);
    _$insert(_el$, count, _el$4);
    _$insert(_el$, () => shared.value, null);
    return _el$;
  })();
});
export { For };
export default Page;
_$delegateEvents(["click"]);
