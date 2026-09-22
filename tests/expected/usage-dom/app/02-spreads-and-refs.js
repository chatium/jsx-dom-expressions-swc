import { template as _$template } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { use as _$use } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<section><h2></h2><p>`);
export function Card(props) {
  let node;
  const rest = () => props.rest;
  return (() => {
    const _el$ = _tmpl$(),
      _el$2 = _el$.firstChild,
      _el$3 = _el$2.nextSibling;
    const _ref$ = node;
    typeof _ref$ === "function" ? _$use(_ref$, _el$) : node = _el$;
    _$spread(_el$, _$mergeProps(rest, {
      get ["data-id"]() {
        return props.id;
      },
      get ["aria-label"]() {
        return props.label || 'card';
      }
    }), false, true);
    _$insert(_el$3, () => props.body);
    _$insert(_el$, () => props.children, null);
    _$effect(() => _el$2.innerHTML = props.title);
    return _el$;
  })();
}
