import { escape as _$escape } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { ssr as _$ssr } from "solid-js/web";
const _tmpl$ = "<div>From Parent</div>",
  _tmpl$2 = ["<div>", "</div>"],
  _tmpl$3 = ["<div>", "", "", "</div>"];
const template = props => {
  let childRef;
  const {
    content
  } = props;
  return _$ssr(_tmpl$3, _$escape(_$createComponent(Child, _$mergeProps({
    name: "John"
  }, props, {
    booleanProperty: true,
    get children() {
      return _$ssr(_tmpl$);
    }
  }))), _$escape(_$createComponent(Child, _$mergeProps({
    name: "Jason"
  }, dynamicSpread, {
    get children() {
      return _$ssr(_tmpl$2, _$escape(content));
    }
  }))), _$escape(_$createComponent(Context.Consumer, {
    children: context => context
  })));
};
