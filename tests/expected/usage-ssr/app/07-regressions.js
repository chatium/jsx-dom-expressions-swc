import { ssrElement as _$ssrElement } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { ssrAttribute as _$ssrAttribute } from "solid-js/web";
import { ssrStyle as _$ssrStyle } from "solid-js/web";
import { ssr as _$ssr } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = ["<div>", "</div>"],
  _tmpl$2 = ["<style>", "</style>"],
  _tmpl$3 = ["<div style=\"", "\"></div>"],
  _tmpl$4 = "<div>x</div>",
  _tmpl$5 = ["<div class=\"", "\"></div>"],
  _tmpl$6 = "<div></div>",
  _tmpl$7 = "<code>\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022</code>",
  _tmpl$8 = "<b>ABCx</b>",
  _tmpl$9 = ["<svg", " viewBox=\"0 0 24 24\"></svg>"],
  _tmpl$0 = ["<img alt=\"logo\" width=\"20\" height=\"20\"", ">"],
  _tmpl$1 = " ",
  _tmpl$10 = "<span>x</span>";
// Shapes that real UGC code hit and the upstream corpus does not. Each one was a live bug.

// A JSX string attribute may span lines; it must be re-escaped when it becomes a JS literal.
// (The spread form is covered in tests/errors.rs: upstream emits unparseable output there.)
const onComponent = _$createComponent(Card, {
  label: "line one\nline two"
});

// `a?.b` is an optional *member*, not an optional call: it must not count as dynamic when
// only call expressions are being checked.
const optionalMember = _$ssr(_tmpl$, isEnabled(ctx) && _$escape(store.workspace?.path));
const optionalCall = _$ssr(_tmpl$, isEnabled(ctx) && _$escape(store.load?.()));

// An expression container holding a single element still has its subtree walked, so the call
// in the component's props makes it dynamic.
const wrapped = _$ssr(_tmpl$, _$escape(_$createComponent(Child, {
  get folderPath() {
    return selected()?.path;
  },
  get folderId() {
    return selected()?.id;
  }
})));

// A const declared *below* its use does not fold: this stays an insert, not template text.
const hoisted = _$ssr(_tmpl$2, LATER_STYLES);
const inOrder = _$ssr(_tmpl$2, EARLIER_STYLES);
const LATER_STYLES = `.a { color: red }`;

// A template literal folds when its substitutions do, so this becomes a plain setProperty.
const ROW_HEIGHT_PX = 72;
const templated = _$ssr(_tmpl$3, "height:" + `${_$escape(ROW_HEIGHT_PX, true)}px`);

// A spread inside an array prop is dynamic: SWC keeps it as a field, not a SpreadElement node.
const spreadInArray = _$createComponent(List, {
  get each() {
    return [...set];
  }
});

// A long multi-line JSX string on a component: it must be rebuilt as a JS literal so the
// printer escapes it. A short one does not reproduce the failure — the length and the mix of
// characters matter, so this keeps the shape of the source that first emitted unparseable
// output.
const realMultiline = _$createComponent(Show, {
  get when() {
    return cond();
  },
  get children() {
    return _$createComponent(Panel, {
      column: true,
      gap: 5,
      style: "\n                padding: 5px 0 5px 10px;\n                margin-left: 17px;\n                margin-top: 10px;\n                color: rgb(0 0 0);\n                font-size: 14px;\n                display: flex;\n                flex-direction: column;\n                gap: 5px;\n                border-left: 3px solid #a1b2c3;\n                background-color: #d4e5f608;\n              ",
      get children() {
        return _$ssr(_tmpl$4);
      }
    });
  }
});

// Loose equality and the other comparison operators fold, so this class is static.
const looseEquality = _$ssr(_tmpl$5, `base ${"dark" == "dark" ? "dark" : ""} ${1 < 2 ? "lt" : ""}`);

// An object folds through a binding with one reference, and stops folding past that: the
// second use cannot know whether the first mutated it.
const onceUsed = {
  "grid-area": "a"
};
const twiceUsed = {
  "grid-area": "b"
};
const foldsOnce = _$ssr(_tmpl$3, _$ssrStyle(onceUsed));
const staysDynamicA = _$ssr(_tmpl$3, _$ssrStyle(twiceUsed));
const staysDynamicB = _$ssr(_tmpl$3, _$ssrStyle(twiceUsed));

// A ref that is neither an lvalue, a function nor a call matches no branch, so nothing is
// emitted for it — and the `use` import must not be registered either, since registration
// order decides how the generated imports are ordered.
const unmatchedRef = _$ssr(_tmpl$6);
const unmatchedRefOnComponent = _$createComponent(C, {});

// Babel's evaluate calls side-effect-free methods on string and number literals.
const literalMethods = _$ssr(_tmpl$7);
const casing = _$ssr(_tmpl$8);

// SSR inlines a literal or binary attribute value into the template, but a logical expression
// goes through ssrAttribute: Babel's isBinaryExpression excludes logical operators.
const ssrLogicalAttr = _$ssr(_tmpl$9, _$ssrAttribute("class", _$escape(props.class, true) || "w-4 h-4", false));

// A numeric attribute keeps its value in the template. The DOM path folds it to a string
// earlier; the SSR path reads the literal directly and must handle a number too.
const numericAttrs = _$ssr(_tmpl$0, _$ssrAttribute("tabindex", -1, false));

// In the SSR spread path the children still sit inside a native element, so a static
// expression among them folds into a template rather than staying an inline string.
const ssrSpreadChildren = _$ssrElement("script", _$mergeProps(() => props.attrs, {
  get id() {
    return props.key;
  }
}), _$ssr(_tmpl$1), false);

// SSR builds the style attribute from the property keys directly, reading `.name` off an
// identifier whether or not the key is computed — so `[cssVar]` contributes the *variable
// name*, and a key that is neither identifier nor literal contributes the string "undefined".
const ssrComputedStyle = _$ssr(_tmpl$3, "color:" + _$escape(c, true) + (";cssVar:" + _$escape(v, true)) + (";--x:" + _$escape(w, true)) + (";undefined:" + _$escape(z, true)));

// An element inside an expression is marked "won't escape", which lets SSR drop the ssr()
// call: a single chunk becomes the template identifier itself, and a chunk pair whose only
// value is the hydration key becomes tmpl[0] + key + tmpl[1].
const wontEscape = _$ssr(_tmpl$, cond ? _tmpl$10 : _$escape(null));
// A namespaced attribute on a component becomes a computed string key.
const namespacedOnComponent = _$createComponent(Comp, {
  "xlink:href": url
});
