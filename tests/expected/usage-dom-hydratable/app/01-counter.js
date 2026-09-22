import { template as _$template } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { runHydrationEvents as _$runHydrationEvents } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { getNextElement as _$getNextElement } from "solid-js/web";
import { getNextMarker as _$getNextMarker } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<ul>`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div class="page"><button><!#><!/><!#><!/>: <!#><!/></button><!#><!/><input type="text">`),
  _tmpl$3 = /*#__PURE__*/_$template(`<em>nothing yet`),
  _tmpl$4 = /*#__PURE__*/_$template(`<li>`);
// The shape an application block takes after TypeScript is stripped.
import { createSignal, createComponent, For, Show } from 'solid-js';
import Widget from './shared/Widget';
export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(0);
  const [items, setItems] = createSignal([]);
  return (() => {
    const _el$ = _$getNextElement(_tmpl$2),
      _el$2 = _el$.firstChild,
      _el$4 = _el$2.firstChild,
      [_el$5, _co$] = _$getNextMarker(_el$4.nextSibling),
      _el$6 = _el$5.nextSibling,
      [_el$7, _co$2] = _$getNextMarker(_el$6.nextSibling),
      _el$3 = _el$7.nextSibling,
      _el$8 = _el$3.nextSibling,
      [_el$9, _co$3] = _$getNextMarker(_el$8.nextSibling),
      _el$10 = _el$2.nextSibling,
      [_el$11, _co$4] = _$getNextMarker(_el$10.nextSibling),
      _el$1 = _el$11.nextSibling;
    _el$2.$$click = () => setCount(count() + 1);
    _$insert(_el$2, _$createComponent(Widget, {}), _el$5, _co$);
    _$insert(_el$2, () => ctx.t('Count'), _el$7, _co$2);
    _$insert(_el$2, count, _el$9, _co$3);
    _$insert(_el$, _$createComponent(Show, {
      get when() {
        return count() > 0;
      },
      get fallback() {
        return _$getNextElement(_tmpl$3);
      },
      get children() {
        const _el$0 = _$getNextElement(_tmpl$);
        _$insert(_el$0, _$createComponent(For, {
          get each() {
            return items();
          },
          children: item => (() => {
            const _el$13 = _$getNextElement(_tmpl$4);
            _$insert(_el$13, () => item.name);
            _$effect(() => _$setAttribute(_el$13, "title", item.name));
            return _el$13;
          })()
        }));
        return _el$0;
      }
    }), _el$11, _co$4);
    _el$1.$$input = e => setItems([...items(), {
      name: e.target.value
    }]);
    _el$1.style.setProperty("margin-top", "4px");
    _$effect(_p$ => {
      const _v$ = !!(items().length === 0),
        _v$2 = count() > 5 ? 'red' : 'inherit';
      _v$ !== _p$._v$ && _el$.classList.toggle("empty", _p$._v$ = _v$);
      _v$2 !== _p$._v$2 && ((_p$._v$2 = _v$2) != null ? _el$1.style.setProperty("color", _v$2) : _el$1.style.removeProperty("color"));
      return _p$;
    }, {
      _v$: undefined,
      _v$2: undefined
    });
    _$effect(() => _el$1.value = count());
    _$runHydrationEvents();
    return _el$;
  })();
});
_$delegateEvents(["click", "input"]);
