import { ssr as _$ssr } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { ssrHydrationKey as _$ssrHydrationKey } from "solid-js/web";
const _tmpl$ = ["<button", "><!--#-->", "<!--/--><!--#-->", "<!--/-->: <!--#-->", "<!--/--> <!--#-->", "<!--/--></button>"];
// A component built by a factory, with a namespace import, a default import and a re-export:
// the shapes an application file mixes around its JSX.
import { createSignal, createComponent, For } from 'solid-js';
import value, * as shared from './shared/value';
import Widget from './shared/Widget';
export const Page = createComponent(ctx => {
  const [count, setCount] = createSignal(value);
  return _$ssr(_tmpl$, _$ssrHydrationKey(), _$escape(_$createComponent(Widget, {})), _$escape(ctx.t('Count')), _$escape(count()), _$escape(shared.value));
});
export { For };
export default Page;
