import { template as _$template } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<ul>`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div class="page"><button>: </button><input type="text">`),
  _tmpl$3 = /*#__PURE__*/_$template(`<em>nothing yet`),
  _tmpl$4 = /*#__PURE__*/_$template(`<li>`);
// The shape an application block takes after TypeScript is stripped.
import { createSignal, createComponent, For, Show } from 'solid-js';
import Widget from './shared/Widget';
export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(0);
  const [items, setItems] = createSignal([]);
  return (() => {
    const _el$ = _tmpl$2(),
      _el$2 = _el$.firstChild,
      _el$3 = _el$2.firstChild,
      _el$5 = _el$2.nextSibling;
    _el$2.$$click = () => setCount(count() + 1);
    _$insert(_el$2, _$createComponent(Widget, {}), _el$3);
    _$insert(_el$2, () => ctx.t('Count'), _el$3);
    _$insert(_el$2, count, null);
    _$insert(_el$, _$createComponent(Show, {
      get when() {
        return count() > 0;
      },
      get fallback() {
        return _tmpl$3();
      },
      get children() {
        const _el$4 = _tmpl$();
        _$insert(_el$4, _$createComponent(For, {
          get each() {
            return items();
          },
          children: item => (() => {
            const _el$7 = _tmpl$4();
            _$insert(_el$7, () => item.name);
            _$effect(() => _$setAttribute(_el$7, "title", item.name));
            return _el$7;
          })()
        }));
        return _el$4;
      }
    }), _el$5);
    _el$5.$$input = e => setItems([...items(), {
      name: e.target.value
    }]);
    _el$5.style.setProperty("margin-top", "4px");
    _$effect(_p$ => {
      const _v$ = !!(items().length === 0),
        _v$2 = count() > 5 ? 'red' : 'inherit';
      _v$ !== _p$._v$ && _el$.classList.toggle("empty", _p$._v$ = _v$);
      _v$2 !== _p$._v$2 && ((_p$._v$2 = _v$2) != null ? _el$5.style.setProperty("color", _v$2) : _el$5.style.removeProperty("color"));
      return _p$;
    }, {
      _v$: undefined,
      _v$2: undefined
    });
    _$effect(() => _el$5.value = count());
    return _el$;
  })();
});
_$delegateEvents(["click", "input"]);
