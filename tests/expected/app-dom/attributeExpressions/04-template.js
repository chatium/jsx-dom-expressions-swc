import { template as _$template } from "solid-js/web";
import { classList as _$classList } from "solid-js/web";
import { use as _$use } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div id="main"><h1 class="base"><a href="/">Welcome`);
const template = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.firstChild;
  _$spread(_el$, _$mergeProps(results, {
    "classList": {
      selected: unknown
    },
    "style": {
      color
    }
  }), false, true);
  _$setAttribute(_el$2, "id", id);
  _$spread(_el$2, _$mergeProps(results, {
    "foo": "",
    "disabled": true,
    get title() {
      return welcoming();
    },
    get style() {
      return {
        "background-color": color(),
        "margin-right": "40px"
      };
    },
    get classList() {
      return {
        dynamic: dynamic(),
        selected
      };
    }
  }), false, true);
  const _ref$ = link;
  typeof _ref$ === "function" ? _$use(_ref$, _el$3) : link = _el$3;
  _$classList(_el$3, {
    "ccc ddd": true
  });
  return _el$;
})();
