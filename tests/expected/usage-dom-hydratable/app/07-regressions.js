import { template as _$template } from "solid-js/web";
import { classList as _$classList } from "solid-js/web";
import { runHydrationEvents as _$runHydrationEvents } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { style as _$style } from "solid-js/web";
import { getNextElement as _$getNextElement } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<div>`),
  _tmpl$2 = /*#__PURE__*/_$template(`<style>`),
  _tmpl$3 = /*#__PURE__*/_$template(`<div>x`),
  _tmpl$4 = /*#__PURE__*/_$template(`<div class="base dark lt">`),
  _tmpl$5 = /*#__PURE__*/_$template(`<code>••••••••••••••••••••••••`),
  _tmpl$6 = /*#__PURE__*/_$template(`<b>ABCx`),
  _tmpl$7 = /*#__PURE__*/_$template(`<svg viewBox="0 0 24 24">`),
  _tmpl$8 = /*#__PURE__*/_$template(`<img alt="logo" width="20" height="20" tabindex="-1">`),
  _tmpl$9 = /*#__PURE__*/_$template(`<script> `),
  _tmpl$0 = /*#__PURE__*/_$template(`<span>x`);
// Shapes that real UGC code hit and the upstream corpus does not. Each one was a live bug.

// A JSX string attribute may span lines; it must be re-escaped when it becomes a JS literal.
// (The spread form is covered in tests/errors.rs: upstream emits unparseable output there.)
const onComponent = _$createComponent(Card, {
  label: "line one\nline two"
});

// `a?.b` is an optional *member*, not an optional call: it must not count as dynamic when
// only call expressions are being checked.
const optionalMember = (() => {
  const _el$ = _$getNextElement(_tmpl$);
  _$insert(_el$, () => isEnabled(ctx) && store.workspace?.path);
  return _el$;
})();
const optionalCall = (() => {
  const _el$2 = _$getNextElement(_tmpl$);
  _$insert(_el$2, (() => {
    const _c$ = _$memo(() => !!isEnabled(ctx));
    return () => _c$() && store.load?.();
  })());
  return _el$2;
})();

// An expression container holding a single element still has its subtree walked, so the call
// in the component's props makes it dynamic.
const wrapped = (() => {
  const _el$3 = _$getNextElement(_tmpl$);
  _$insert(_el$3, () => _$createComponent(Child, {
    get folderPath() {
      return selected()?.path;
    },
    get folderId() {
      return selected()?.id;
    }
  }));
  return _el$3;
})();

// A const declared *below* its use does not fold: this stays an insert, not template text.
const hoisted = (() => {
  const _el$4 = _$getNextElement(_tmpl$2);
  _$insert(_el$4, LATER_STYLES);
  return _el$4;
})();
const inOrder = (() => {
  const _el$5 = _$getNextElement(_tmpl$2);
  _$insert(_el$5, EARLIER_STYLES);
  return _el$5;
})();
const LATER_STYLES = `.a { color: red }`;

// A template literal folds when its substitutions do, so this becomes a plain setProperty.
const ROW_HEIGHT_PX = 72;
const templated = (() => {
  const _el$6 = _$getNextElement(_tmpl$);
  _el$6.style.setProperty("height", "72px");
  return _el$6;
})();

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
        return _$getNextElement(_tmpl$3);
      }
    });
  }
});

// Loose equality and the other comparison operators fold, so this class is static.
const looseEquality = _$getNextElement(_tmpl$4);

// An object folds through a binding with one reference, and stops folding past that: the
// second use cannot know whether the first mutated it.
const onceUsed = {
  "grid-area": "a"
};
const twiceUsed = {
  "grid-area": "b"
};
const foldsOnce = (() => {
  const _el$9 = _$getNextElement(_tmpl$);
  _$style(_el$9, onceUsed);
  return _el$9;
})();
const staysDynamicA = (() => {
  const _el$0 = _$getNextElement(_tmpl$);
  _$effect(_$p => _$style(_el$0, twiceUsed, _$p));
  return _el$0;
})();
const staysDynamicB = (() => {
  const _el$1 = _$getNextElement(_tmpl$);
  _$effect(_$p => _$style(_el$1, twiceUsed, _$p));
  return _el$1;
})();

// A ref that is neither an lvalue, a function nor a call matches no branch, so nothing is
// emitted for it — and the `use` import must not be registered either, since registration
// order decides how the generated imports are ordered.
const unmatchedRef = _$getNextElement(_tmpl$);
const unmatchedRefOnComponent = _$createComponent(C, {});

// Babel's evaluate calls side-effect-free methods on string and number literals.
const literalMethods = _$getNextElement(_tmpl$5);
const casing = _$getNextElement(_tmpl$6);

// SSR inlines a literal or binary attribute value into the template, but a logical expression
// goes through ssrAttribute: Babel's isBinaryExpression excludes logical operators.
const ssrLogicalAttr = (() => {
  const _el$13 = _$getNextElement(_tmpl$7);
  _$effect(() => _$setAttribute(_el$13, "class", props.class || "w-4 h-4"));
  return _el$13;
})();

// A numeric attribute keeps its value in the template. The DOM path folds it to a string
// earlier; the SSR path reads the literal directly and must handle a number too.
const numericAttrs = _$getNextElement(_tmpl$8);

// In the SSR spread path the children still sit inside a native element, so a static
// expression among them folds into a template rather than staying an inline string.
const ssrSpreadChildren = (() => {
  const _el$15 = _$getNextElement(_tmpl$9);
  _$spread(_el$15, _$mergeProps(() => props.attrs, {
    get id() {
      return props.key;
    }
  }), false, true);
  _$runHydrationEvents();
  return _el$15;
})();

// SSR builds the style attribute from the property keys directly, reading `.name` off an
// identifier whether or not the key is computed — so `[cssVar]` contributes the *variable
// name*, and a key that is neither identifier nor literal contributes the string "undefined".
const ssrComputedStyle = (() => {
  const _el$16 = _$getNextElement(_tmpl$);
  c != null ? _el$16.style.setProperty("color", c) : _el$16.style.removeProperty("color");
  _$effect(_$p => _$style(_el$16, {
    color: c,
    [cssVar]: v,
    [f()]: z
  }, _$p));
  return _el$16;
})();

// An element inside an expression is marked "won't escape", which lets SSR drop the ssr()
// call: a single chunk becomes the template identifier itself, and a chunk pair whose only
// value is the hydration key becomes tmpl[0] + key + tmpl[1].
const wontEscape = (() => {
  const _el$17 = _$getNextElement(_tmpl$);
  _$insert(_el$17, cond ? _$getNextElement(_tmpl$0) : null);
  return _el$17;
})();
// A namespaced attribute on a component becomes a computed string key.
const namespacedOnComponent = _$createComponent(Comp, {
  "xlink:href": url
});

// A computed key does not by itself make an object unevaluable: Babel evaluates the key and
// stays confident, so a constant one keeps style/classList static instead of reactive.
const computedKeyStatic = (() => {
  const _el$19 = _$getNextElement(_tmpl$);
  _$style(_el$19, {
    ['flex-direction']: 'row'
  });
  return _el$19;
})();
const computedKeyFolded = (() => {
  const _el$20 = _$getNextElement(_tmpl$);
  _$style(_el$20, {
    ['flex-' + 'direction']: 'row'
  });
  return _el$20;
})();
const computedKeyClassList = (() => {
  const _el$21 = _$getNextElement(_tmpl$);
  _$classList(_el$21, {
    ['is-open']: true
  });
  return _el$21;
})();
// An unevaluable key still deopts the whole object.
const computedKeyDynamic = (() => {
  const _el$22 = _$getNextElement(_tmpl$);
  _$effect(_$p => _$style(_el$22, {
    [dynamic]: 'row'
  }, _$p));
  return _el$22;
})();

// Shorthand is a key/value property to Babel, so it evaluates like one. This object skips the
// classList preprocessing (a key with a space), which is what routes it through evaluate().
const shorthandActive = true;
const shorthandInObject = (() => {
  const _el$23 = _$getNextElement(_tmpl$);
  _$classList(_el$23, {
    'a b': true,
    shorthandActive
  });
  return _el$23;
})();
