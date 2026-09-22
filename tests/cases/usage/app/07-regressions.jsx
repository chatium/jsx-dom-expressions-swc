// Shapes that real UGC code hit and the upstream corpus does not. Each one was a live bug.

// A JSX string attribute may span lines; it must be re-escaped when it becomes a JS literal.
// (The spread form is covered in tests/errors.rs: upstream emits unparseable output there.)
const onComponent = <Card label="line one
line two" />

// `a?.b` is an optional *member*, not an optional call: it must not count as dynamic when
// only call expressions are being checked.
const optionalMember = <div>{isEnabled(ctx) && store.workspace?.path}</div>
const optionalCall = <div>{isEnabled(ctx) && store.load?.()}</div>

// An expression container holding a single element still has its subtree walked, so the call
// in the component's props makes it dynamic.
const wrapped = (
  <div>
    {
      <Child folderPath={selected()?.path} folderId={selected()?.id} />
    }
  </div>
)

// A const declared *below* its use does not fold: this stays an insert, not template text.
const hoisted = <style>{LATER_STYLES}</style>
const inOrder = <style>{EARLIER_STYLES}</style>
const LATER_STYLES = `.a { color: red }`

// A template literal folds when its substitutions do, so this becomes a plain setProperty.
const ROW_HEIGHT_PX = 72
const templated = <div style:height={`${ROW_HEIGHT_PX}px`} />

// A spread inside an array prop is dynamic: SWC keeps it as a field, not a SpreadElement node.
const spreadInArray = <List each={[...set]} />

// A long multi-line JSX string on a component: it must be rebuilt as a JS literal so the
// printer escapes it. A short one does not reproduce the failure — the length and the mix of
// characters matter, so this keeps the shape of the source that first emitted unparseable
// output.
const realMultiline = <Show when={cond()}>
            <Panel column gap={5} style="
                padding: 5px 0 5px 10px;
                margin-left: 17px;
                margin-top: 10px;
                color: rgb(0 0 0);
                font-size: 14px;
                display: flex;
                flex-direction: column;
                gap: 5px;
                border-left: 3px solid #a1b2c3;
                background-color: #d4e5f608;
              ">
              <div>x</div>
            </Panel>
          </Show>

// Loose equality and the other comparison operators fold, so this class is static.
const looseEquality = <div class={`base ${"dark" == "dark" ? "dark" : ""} ${1 < 2 ? "lt" : ""}`} />

// An object folds through a binding with one reference, and stops folding past that: the
// second use cannot know whether the first mutated it.
const onceUsed = {"grid-area": "a"}
const twiceUsed = {"grid-area": "b"}
const foldsOnce = <div style={onceUsed} />
const staysDynamicA = <div style={twiceUsed} />
const staysDynamicB = <div style={twiceUsed} />

// A ref that is neither an lvalue, a function nor a call matches no branch, so nothing is
// emitted for it — and the `use` import must not be registered either, since registration
// order decides how the generated imports are ordered.
const unmatchedRef = <div ref={x || y} />
const unmatchedRefOnComponent = <C ref={x || y} />

// Babel's evaluate calls side-effect-free methods on string and number literals.
const literalMethods = <code>{"•".repeat(24)}</code>
const casing = <b>{"abc".toUpperCase()}{"  x  ".trim()}</b>

// SSR inlines a literal or binary attribute value into the template, but a logical expression
// goes through ssrAttribute: Babel's isBinaryExpression excludes logical operators.
const ssrLogicalAttr = <svg class={props.class || "w-4 h-4"} viewBox="0 0 24 24" />

// A numeric attribute keeps its value in the template. The DOM path folds it to a string
// earlier; the SSR path reads the literal directly and must handle a number too.
const numericAttrs = <img alt="logo" width={20} height={20} tabindex={-1} />

// In the SSR spread path the children still sit inside a native element, so a static
// expression among them folds into a template rather than staying an inline string.
const ssrSpreadChildren = <script {...props.attrs} id={props.key}>{" "}</script>

// SSR builds the style attribute from the property keys directly, reading `.name` off an
// identifier whether or not the key is computed — so `[cssVar]` contributes the *variable
// name*, and a key that is neither identifier nor literal contributes the string "undefined".
const ssrComputedStyle = <div style={{color: c, [cssVar]: v, ["--x"]: w, [f()]: z}} />

// An element inside an expression is marked "won't escape", which lets SSR drop the ssr()
// call: a single chunk becomes the template identifier itself, and a chunk pair whose only
// value is the hydration key becomes tmpl[0] + key + tmpl[1].
const wontEscape = <div>{cond ? <span>x</span> : null}</div>
// A namespaced attribute on a component becomes a computed string key.
const namespacedOnComponent = <Comp xlink:href={url} />

// A computed key does not by itself make an object unevaluable: Babel evaluates the key and
// stays confident, so a constant one keeps style/classList static instead of reactive.
const computedKeyStatic = <div style={{ ['flex-direction']: 'row' }} />
const computedKeyFolded = <div style={{ ['flex-' + 'direction']: 'row' }} />
const computedKeyClassList = <div classList={{ ['is-open']: true }} />
// An unevaluable key still deopts the whole object.
const computedKeyDynamic = <div style={{ [dynamic]: 'row' }} />

// Shorthand is a key/value property to Babel, so it evaluates like one. This object skips the
// classList preprocessing (a key with a space), which is what routes it through evaluate().
const shorthandActive = true
const shorthandInObject = <div classList={{ 'a b': true, shorthandActive }} />
