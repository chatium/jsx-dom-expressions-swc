import { escape as _$escape } from "r-server";
import { createComponent as _$createComponent } from "r-server";
import { mergeProps as _$mergeProps } from "r-server";
import { ssr as _$ssr } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<div", ">From Parent</div>"],
  _tmpl$2 = ["<div", ">", "</div>"],
  _tmpl$3 = ["<div", "><!--#-->", "<!--/--><!--#-->", "<!--/--><!--#-->", "<!--/--></div>"];
const template = props => {
  let childRef;
  const {
    content
  } = props;
  return _$ssr(_tmpl$3, _$ssrHydrationKey(), _$escape(_$createComponent(Child, _$mergeProps({
    name: "John"
  }, props, {
    booleanProperty: true,
    get children() {
      return _$ssr(_tmpl$, _$ssrHydrationKey());
    }
  }))), _$escape(_$createComponent(Child, _$mergeProps({
    name: "Jason"
  }, dynamicSpread, {
    get children() {
      return _$ssr(_tmpl$2, _$ssrHydrationKey(), _$escape(content));
    }
  }))), _$escape(_$createComponent(Context.Consumer, {
    children: context => context
  })));
};
