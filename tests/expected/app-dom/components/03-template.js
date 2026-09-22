import { template as _$template } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>From Parent`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div>`);
const template = props => {
  let childRef;
  const {
    content
  } = props;
  return (() => {
    const _el$ = _tmpl$2();
    _$insert(_el$, _$createComponent(Child, _$mergeProps({
      name: "John"
    }, props, {
      ref(r$) {
        const _ref$ = childRef;
        typeof _ref$ === "function" ? _ref$(r$) : childRef = r$;
      },
      booleanProperty: true,
      get children() {
        return _tmpl$();
      }
    })), null);
    _$insert(_el$, _$createComponent(Child, _$mergeProps({
      name: "Jason"
    }, dynamicSpread, {
      ref(r$) {
        const _ref$2 = props.ref;
        typeof _ref$2 === "function" ? _ref$2(r$) : props.ref = r$;
      },
      get children() {
        const _el$3 = _tmpl$2();
        _$insert(_el$3, content);
        return _el$3;
      }
    })), null);
    _$insert(_el$, _$createComponent(Context.Consumer, {
      ref(r$) {
        const _ref$3 = props.consumerRef();
        typeof _ref$3 === "function" && _ref$3(r$);
      },
      children: context => context
    }), null);
    return _el$;
  })();
};
