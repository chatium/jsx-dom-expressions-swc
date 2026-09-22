import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
const _tmpl$ = ["<div>Hello ", "</div>"],
  _tmpl$2 = ["<div>", "</div>"];
const Child = props => {
  const [s, set] = createSignal();
  return [_$ssr(_tmpl$, _$escape(props.name)), _$ssr(_tmpl$2, _$escape(props.children))];
};
