import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { ssr as _$ssr } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = ["<ul>", "</ul>"],
  _tmpl$2 = ["<div class=\"", "\"><button>", "", ": ", "</button>", "<input type=\"text\"", " style=\"", "\"></div>"],
  _tmpl$3 = "<em>nothing yet</em>",
  _tmpl$4 = ["<li", ">", "</li>"];
// The shape an application block takes after TypeScript is stripped.
import { createSignal, createComponent, For, Show } from 'solid-js';
import Widget from './shared/Widget';
export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(0);
  const [items, setItems] = createSignal([]);
  return _$ssr(_tmpl$2, `page ${items().length === 0 ? "empty" : ""}`, _$escape(_$createComponent(Widget, {})), _$escape(ctx.t('Count')), _$escape(count()), _$escape(_$createComponent(Show, {
    get when() {
      return count() > 0;
    },
    get fallback() {
      return _$ssr(_tmpl$3);
    },
    get children() {
      return _$ssr(_tmpl$, _$escape(_$createComponent(For, {
        get each() {
          return items();
        },
        children: item => _$ssr(_tmpl$4, _$ssrAttribute("title", _$escape(item.name, true), false), _$escape(item.name))
      })));
    }
  })), _$ssrAttribute("value", _$escape(count(), true), false), "color:" + (count() > 5 ? "red" : "inherit") + (";margin-top:" + "4px"));
});
