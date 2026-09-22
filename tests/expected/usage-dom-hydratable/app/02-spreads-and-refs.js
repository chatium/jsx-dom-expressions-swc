import { template as _$template } from "solid-js/web";
import { innerHTML as _$innerHTML } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { getNextElement as _$getNextElement } from "solid-js/web";
import { runHydrationEvents as _$runHydrationEvents } from "solid-js/web";
import { getNextMarker as _$getNextMarker } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { use as _$use } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<section><h2></h2><p></p><!#><!/>`);
export function Card(props) {
  let node;
  const rest = () => props.rest;
  return (() => {
    const _el$ = _$getNextElement(_tmpl$),
      _el$2 = _el$.firstChild,
      _el$3 = _el$2.nextSibling,
      _el$4 = _el$3.nextSibling,
      [_el$5, _co$] = _$getNextMarker(_el$4.nextSibling);
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
    _$insert(_el$, () => props.children, _el$5, _co$);
    _$effect(() => _$innerHTML(_el$2, props.title));
    _$runHydrationEvents();
    return _el$;
  })();
}
