import { template as _$template } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`),
  _tmpl$2 = /*#__PURE__*/_$template(`<module>`),
  _tmpl$3 = /*#__PURE__*/_$template(`<module>Hello`),
  _tmpl$4 = /*#__PURE__*/_$template(`<module>Hi `),
  _tmpl$5 = /*#__PURE__*/_$template(`<module>Hi`),
  _tmpl$6 = /*#__PURE__*/_$template(`<div>Test 1`);
const children = _tmpl$();
const dynamic = {
  children
};
const template = _$createComponent(Module, {
  children: children
});
const template2 = (() => {
  const _el$2 = _tmpl$2();
  _$insert(_el$2, children);
  return _el$2;
})();
const template3 = _tmpl$3();
const template4 = (() => {
  const _el$4 = _tmpl$2();
  _$insert(_el$4, _$createComponent(Hello, {}));
  return _el$4;
})();
const template5 = (() => {
  const _el$5 = _tmpl$2();
  _$insert(_el$5, () => dynamic.children);
  return _el$5;
})();
const template6 = _$createComponent(Module, {
  get children() {
    return dynamic.children;
  }
});
const template7 = (() => {
  const _el$6 = _tmpl$2();
  _$spread(_el$6, dynamic, false, false);
  return _el$6;
})();
const template8 = (() => {
  const _el$7 = _tmpl$3();
  _$spread(_el$7, dynamic, false, true);
  return _el$7;
})();
const template9 = (() => {
  const _el$8 = _tmpl$2();
  _$spread(_el$8, dynamic, false, true);
  _$insert(_el$8, () => dynamic.children);
  return _el$8;
})();
const template10 = _$createComponent(Module, _$mergeProps(dynamic, {
  children: "Hello"
}));
const template11 = (() => {
  const _el$9 = _tmpl$2();
  _$insert(_el$9, state.children);
  return _el$9;
})();
const template12 = _$createComponent(Module, {
  children: state.children
});
const template13 = (() => {
  const _el$0 = _tmpl$2();
  _$insert(_el$0, children);
  return _el$0;
})();
const template14 = _$createComponent(Module, {
  children: children
});
const template15 = (() => {
  const _el$1 = _tmpl$2();
  _$insert(_el$1, () => dynamic.children);
  return _el$1;
})();
const template16 = _$createComponent(Module, {
  get children() {
    return dynamic.children;
  }
});
const template18 = (() => {
  const _el$10 = _tmpl$4();
  _$insert(_el$10, children, null);
  return _el$10;
})();
const template19 = _$createComponent(Module, {
  get children() {
    return ["Hi ", children];
  }
});
const template20 = (() => {
  const _el$11 = _tmpl$2();
  _$insert(_el$11, children);
  return _el$11;
})();
const template21 = _$createComponent(Module, {
  get children() {
    return children();
  }
});
const template22 = (() => {
  const _el$12 = _tmpl$2();
  _$insert(_el$12, () => state.children());
  return _el$12;
})();
const template23 = _$createComponent(Module, {
  get children() {
    return state.children();
  }
});
const template24 = (() => {
  const _el$13 = _tmpl$5(),
    _el$14 = _el$13.firstChild;
  _$spread(_el$13, dynamic, false, true);
  _$insert(_el$13, () => dynamic.children, null);
  return _el$13;
})();
const tiles = [];
tiles.push(_tmpl$6());
const template25 = (() => {
  const _el$16 = _tmpl$();
  _$insert(_el$16, tiles);
  return _el$16;
})();
const comma = (() => {
  const _el$17 = _tmpl$();
  _$insert(_el$17, () => (expression(), "static"));
  return _el$17;
})();
