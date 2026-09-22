import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", ">Hello <!--#-->", "<!--/--></div>"],
  _tmpl$2 = ["<div", ">", "</div>"];
const Child = props => {
  const [s, set] = createSignal();
  return [_$ssr(_tmpl$, _$ssrHydrationKey(), _$escape(props.name)), _$ssr(_tmpl$2, _$ssrHydrationKey(), _$escape(props.children))];
};
