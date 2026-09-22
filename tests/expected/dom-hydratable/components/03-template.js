import { template as _$template } from "r-dom";
import { getNextMarker as _$getNextMarker } from "r-dom";
import { insert as _$insert } from "r-dom";
import { createComponent as _$createComponent } from "r-dom";
import { mergeProps as _$mergeProps } from "r-dom";
import { getNextElement as _$getNextElement } from "r-dom";
const _tmpl$ = /*#__PURE__*/_$template(`<div>From Parent`),
  _tmpl$2 = /*#__PURE__*/_$template(`<div>`),
  _tmpl$3 = /*#__PURE__*/_$template(`<div><!#><!/><!#><!/><!#><!/>`);
const template = props => {
  let childRef;
  const {
    content
  } = props;
  return (() => {
    const _el$ = _$getNextElement(_tmpl$3),
      _el$4 = _el$.firstChild,
      [_el$5, _co$] = _$getNextMarker(_el$4.nextSibling),
      _el$6 = _el$5.nextSibling,
      [_el$7, _co$2] = _$getNextMarker(_el$6.nextSibling),
      _el$8 = _el$7.nextSibling,
      [_el$9, _co$3] = _$getNextMarker(_el$8.nextSibling);
    _$insert(_el$, _$createComponent(Child, _$mergeProps({
      name: "John"
    }, props, {
      ref(r$) {
        const _ref$ = childRef;
        typeof _ref$ === "function" ? _ref$(r$) : childRef = r$;
      },
      booleanProperty: true,
      get children() {
        return _$getNextElement(_tmpl$);
      }
    })), _el$5, _co$);
    _$insert(_el$, _$createComponent(Child, _$mergeProps({
      name: "Jason"
    }, dynamicSpread, {
      ref(r$) {
        const _ref$2 = props.ref;
        typeof _ref$2 === "function" ? _ref$2(r$) : props.ref = r$;
      },
      get children() {
        const _el$3 = _$getNextElement(_tmpl$2);
        _$insert(_el$3, content);
        return _el$3;
      }
    })), _el$7, _co$2);
    _$insert(_el$, _$createComponent(Context.Consumer, {
      ref(r$) {
        const _ref$3 = props.consumerRef();
        typeof _ref$3 === "function" && _ref$3(r$);
      },
      children: context => context
    }), _el$9, _co$3);
    return _el$;
  })();
};
