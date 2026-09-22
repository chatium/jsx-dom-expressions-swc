import { ssrElement as _$ssrElement } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
import { ssr as _$ssr } from "solid-js/web";
const _tmpl$ = ["<h2>", "</h2>"],
  _tmpl$2 = ["<p>", "</p>"];
export function Card(props) {
  let node;
  const rest = () => props.rest;
  return _$ssrElement("section", _$mergeProps(rest, {
    get ["data-id"]() {
      return props.id;
    },
    get ["aria-label"]() {
      return props.label || 'card';
    }
  }), () => [_$ssr(_tmpl$, props.title), _$ssr(_tmpl$2, _$escape(props.body)), "<!--#-->", _$escape(props.children), "<!--/-->"], true);
}
