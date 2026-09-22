use std::borrow::Cow;

use swc_core::common::comments::Comments;
use swc_core::common::{DUMMY_SP, Span, Spanned};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::VisitWith;

use crate::Transform;
use crate::constants::{NON_SPREAD_NAMESPACES, RESERVED_NAMESPACES};
use crate::eval::{Value, evaluate};

// ---------------------------------------------------------------- names

pub(crate) fn jsx_element_name_to_string(name: &JSXElementName) -> String {
    match name {
        JSXElementName::Ident(ident) => ident.sym.to_string(),
        JSXElementName::JSXMemberExpr(member) => {
            format!("{}.{}", jsx_object_to_string(&member.obj), member.prop.sym)
        }
        JSXElementName::JSXNamespacedName(ns) => format!("{}:{}", ns.ns.sym, ns.name.sym),
    }
}

fn jsx_object_to_string(obj: &JSXObject) -> String {
    match obj {
        JSXObject::Ident(ident) => ident.sym.to_string(),
        JSXObject::JSXMemberExpr(member) => {
            format!("{}.{}", jsx_object_to_string(&member.obj), member.prop.sym)
        }
    }
}

pub(crate) fn get_tag_name(el: &JSXElement) -> String {
    jsx_element_name_to_string(&el.opening.name)
}

pub(crate) fn is_component(tag_name: &str) -> bool {
    let Some(first) = tag_name.chars().next() else {
        return false;
    };
    !first.is_lowercase() && first.to_lowercase().to_string() != first.to_string()
        || tag_name.contains('.')
        || !first.is_ascii_alphabetic()
}

pub(crate) fn attr_key(name: &JSXAttrName) -> String {
    match name {
        JSXAttrName::Ident(ident) => ident.sym.to_string(),
        JSXAttrName::JSXNamespacedName(ns) => format!("{}:{}", ns.ns.sym, ns.name.sym),
    }
}

/// `convertJSXIdentifier` for an attribute name, as an object property key.
pub(crate) fn convert_jsx_identifier(name: &JSXAttrName) -> PropName {
    match name {
        JSXAttrName::Ident(ident) if is_valid_identifier(&ident.sym) => {
            PropName::Ident(ident.clone())
        }
        JSXAttrName::Ident(ident) => PropName::Str(Str {
            span: DUMMY_SP,
            value: ident.sym.as_str().into(),
            raw: None,
        }),
        JSXAttrName::JSXNamespacedName(ns) => PropName::Str(Str {
            span: DUMMY_SP,
            value: format!("{}:{}", ns.ns.sym, ns.name.sym).into(),
            raw: None,
        }),
    }
}

/// The `key` Babel reads off a converted identifier: `undefined` unless it stayed an identifier.
pub(crate) fn prop_key_name(key: &PropName) -> Option<&str> {
    match key {
        PropName::Ident(ident) => Some(&ident.sym),
        _ => None,
    }
}

static KEYWORDS: &[&str] = &[
    "break",
    "case",
    "catch",
    "continue",
    "debugger",
    "default",
    "do",
    "else",
    "finally",
    "for",
    "function",
    "if",
    "return",
    "switch",
    "throw",
    "try",
    "var",
    "const",
    "while",
    "with",
    "new",
    "this",
    "super",
    "class",
    "extends",
    "export",
    "import",
    "null",
    "true",
    "false",
    "in",
    "instanceof",
    "typeof",
    "void",
    "delete",
];

static RESERVED: &[&str] = &[
    "implements",
    "interface",
    "let",
    "package",
    "private",
    "protected",
    "public",
    "static",
    "yield",
    "await",
    "enum",
];

pub(crate) fn is_valid_identifier(name: &str) -> bool {
    if KEYWORDS.contains(&name) || RESERVED.contains(&name) {
        return false;
    }
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c == '$' || c == '_' || c.is_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c == '$' || c == '_' || c.is_alphanumeric())
}

// ---------------------------------------------------------------- text

/// JavaScript's `\s` character class, which is wider than `char::is_whitespace`.
fn is_js_space(c: char) -> bool {
    matches!(
        c,
        '\u{9}'..='\u{d}'
            | ' '
            | '\u{a0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200a}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202f}'
            | '\u{205f}'
            | '\u{3000}'
            | '\u{feff}'
    )
}

pub(crate) fn trim_whitespace(text: &str) -> String {
    // Most JSX text has no carriage return and no run of whitespace to collapse; checking is
    // cheaper than rebuilding the string twice.
    if !text.contains('\r')
        && !text.contains('\n')
        && !text
            .as_bytes()
            .windows(2)
            .any(|w| is_js_space(w[0] as char) && is_js_space(w[1] as char))
        && text.is_ascii()
    {
        return text.to_string();
    }
    let text = text.replace('\r', "");
    let text = if text.contains('\n') {
        text.split('\n')
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    line.to_string()
                } else {
                    line.trim_start_matches(is_js_space).to_string()
                }
            })
            .filter(|line| !line.chars().all(is_js_space))
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        text
    };
    // .replace(/\s+/g, " ")
    let mut out = String::with_capacity(text.len());
    let mut in_space = false;
    for c in text.chars() {
        if is_js_space(c) {
            if !in_space {
                out.push(' ');
                in_space = true;
            }
        } else {
            out.push(c);
            in_space = false;
        }
    }
    out
}

pub(crate) fn escape_backticks(value: &str) -> Cow<'_, str> {
    if value.contains('`') {
        Cow::Owned(value.replace('`', "\\`"))
    } else {
        Cow::Borrowed(value)
    }
}

/// Upstream returns the input untouched when there is nothing to escape, and most template
/// text has nothing. Borrowing in that case keeps this off the allocation path.
pub(crate) fn escape_html(s: &str, attr: bool) -> Cow<'_, str> {
    let delim = if attr { '"' } else { '<' };
    if !s.contains(delim) && !s.contains('&') {
        return Cow::Borrowed(s);
    }
    let esc_delim = if attr { "&quot;" } else { "&lt;" };
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        if c == delim {
            out.push_str(esc_delim);
        } else if c == '&' {
            out.push_str("&amp;");
        } else {
            out.push(c);
        }
    }
    Cow::Owned(out)
}

pub(crate) fn to_event_name(name: &str) -> String {
    name[2..].to_lowercase()
}

pub(crate) fn to_property_name(name: &str) -> String {
    let lower = name.to_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut chars = lower.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '-' {
            match chars.peek() {
                Some(next) if next.is_ascii_lowercase() => {
                    out.push(next.to_ascii_uppercase());
                    chars.next();
                }
                _ => out.push(c),
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub(crate) fn can_native_spread(key: &str, check_namespaces: bool) -> bool {
    if check_namespaces
        && key.contains(':')
        && NON_SPREAD_NAMESPACES.contains(&key.split(':').next().unwrap())
    {
        return false;
    }
    // TODO (upstream): figure out how to detect a definitely-function ref
    key != "ref"
}

pub(crate) fn is_reserved_namespace(name: &JSXAttrName) -> bool {
    match name {
        JSXAttrName::JSXNamespacedName(ns) => RESERVED_NAMESPACES.contains(&&*ns.ns.sym),
        _ => false,
    }
}

// ---------------------------------------------------------------- children

pub(crate) fn is_empty_expr_container(child: &JSXElementChild) -> bool {
    matches!(
        child,
        JSXElementChild::JSXExprContainer(JSXExprContainer {
            expr: JSXExpr::JSXEmptyExpr(_),
            ..
        })
    )
}

fn jsx_text_raw(text: &JSXText) -> &str {
    &text.raw
}

/// Whether `filterChildren` keeps this child.
pub(crate) fn is_kept_child(child: &JSXElementChild) -> bool {
    if is_empty_expr_container(child) {
        return false;
    }
    match child {
        JSXElementChild::JSXText(text) => {
            let raw = jsx_text_raw(text);
            !(raw.starts_with(['\r', '\n']) && raw[1..].chars().all(|c| c.is_whitespace()))
        }
        _ => true,
    }
}

/// `filterChildren` without taking ownership, for the analysis passes that only read.
pub(crate) fn filter_children_ref(children: &[JSXElementChild]) -> Vec<&JSXElementChild> {
    children
        .iter()
        .filter(|child| is_kept_child(child))
        .collect()
}

/// `filterChildren`: drop empty expression containers and pure line-break text.
pub(crate) fn filter_children(children: Vec<JSXElementChild>) -> Vec<JSXElementChild> {
    children
        .into_iter()
        .filter(|child| {
            if is_empty_expr_container(child) {
                return false;
            }
            match child {
                JSXElementChild::JSXText(text) => {
                    let raw = jsx_text_raw(text);
                    !(raw.starts_with(['\r', '\n']) && raw[1..].chars().all(|c| c.is_whitespace()))
                }
                _ => true,
            }
        })
        .collect()
}

/// `checkLength`: more than one child that is neither empty nor blank-but-not-spaces text.
pub(crate) fn check_length(children: &[JSXElementChild]) -> bool {
    let mut count = 0;
    for child in children {
        if is_empty_expr_container(child) {
            continue;
        }
        if let JSXElementChild::JSXText(text) = child {
            let raw = jsx_text_raw(text);
            let blank = raw.chars().all(|c| c.is_whitespace());
            let spaces_only = raw.chars().all(|c| c == ' ');
            if blank && !spaces_only {
                continue;
            }
        }
        count += 1;
    }
    count > 1
}

// ---------------------------------------------------------------- dynamic checks

#[derive(Clone, Copy, Default)]
pub(crate) struct DynamicOpts {
    pub(crate) check_member: bool,
    pub(crate) check_tags: bool,
    pub(crate) check_call_expressions: bool,
    /// Set for an expression in native-element position: under SSR nothing is reactive there,
    /// so member access and calls alone do not make it dynamic.
    pub(crate) native: bool,
}

impl DynamicOpts {
    pub(crate) fn new() -> Self {
        Self {
            check_call_expressions: true,
            ..Default::default()
        }
    }
    pub(crate) fn member(mut self) -> Self {
        self.check_member = true;
        self
    }
    pub(crate) fn native(mut self, native: bool) -> Self {
        self.native = native;
        self
    }
    pub(crate) fn tags(mut self) -> Self {
        self.check_tags = true;
        self
    }
}

impl<C: Comments> Transform<C> {
    /// The `@once` marker: Babel shifts the comment off the node, so a second `isDynamic`
    /// on the same expression no longer sees it. Taking it out of the store does the same.
    fn take_static_marker(&self, span: Span) -> bool {
        let Some(mut comments) = self.comments.take_leading(span.lo) else {
            return false;
        };
        let matched = comments
            .first()
            .is_some_and(|comment| comment.text.trim() == self.config.static_marker);
        if matched {
            comments.remove(0);
        }
        if !comments.is_empty() {
            self.comments.add_leading_comments(span.lo, comments);
        }
        matched
    }

    pub(crate) fn is_dynamic(&self, expr: &Expr, mut opts: DynamicOpts) -> bool {
        if self.config.generate == crate::Generate::Ssr && opts.native {
            opts.check_member = false;
            opts.check_call_expressions = false;
        }
        if matches!(expr, Expr::Fn(_) | Expr::Arrow(_)) {
            return false;
        }
        if self.take_static_marker(expr.span()) {
            return false;
        }
        // Babel splits `a?.()` (OptionalCallExpression, gated by checkCallExpressions) from
        // `a?.b` (OptionalMemberExpression, gated by checkMember); SWC folds both into
        // `OptChain`, so the base has to be inspected or `a?.b` counts as a call.
        let optional_call = matches!(expr, Expr::OptChain(chain)
            if matches!(&*chain.base, OptChainBase::Call(_)));
        let optional_member = matches!(expr, Expr::OptChain(chain)
            if matches!(&*chain.base, OptChainBase::Member(_)));
        if (opts.check_call_expressions && (matches!(expr, Expr::Call(_)) || optional_call))
            || (opts.check_member
                && (matches!(expr, Expr::Member(_) | Expr::SuperProp(_))
                    || optional_member
                    || matches!(expr, Expr::Bin(bin) if bin.op == BinaryOp::In)))
            || (opts.check_tags && matches!(expr, Expr::JSXElement(_) | Expr::JSXFragment(_)))
        {
            return true;
        }
        let mut finder = DynamicFinder {
            opts,
            found: false,
            transform: self,
        };
        // Babel's `path.traverse` visits the node's children, so the root's own visitor never
        // runs: a root JSXElement has its subtree walked even though a *nested* one is skipped.
        // Unwrapping it here reproduces that asymmetry.
        match expr {
            Expr::JSXElement(el) => el.visit_children_with(&mut finder),
            Expr::JSXFragment(fragment) => fragment.visit_children_with(&mut finder),
            other => other.visit_children_with(&mut finder),
        }
        finder.found
    }
}

/// Babel traverses the node's *children* with `skip()` on functions and (when not checking
/// tags) on nested JSX. This mirrors that walk.
struct DynamicFinder<'a, C: Comments> {
    opts: DynamicOpts,
    found: bool,
    transform: &'a Transform<C>,
}

impl<C: Comments> swc_core::ecma::visit::Visit for DynamicFinder<'_, C> {
    fn visit_function(&mut self, node: &Function) {
        let _ = node; // Babel skips function bodies entirely.
    }

    fn visit_arrow_expr(&mut self, node: &ArrowExpr) {
        let _ = node;
    }

    fn visit_class_method(&mut self, node: &ClassMethod) {
        let _ = node;
    }

    fn visit_method_prop(&mut self, node: &MethodProp) {
        // An object method: Babel still checks a computed key.
        if let PropName::Computed(computed) = &node.key
            && self.transform.is_dynamic(&computed.expr, self.opts)
        {
            self.found = true;
        }
    }

    fn visit_getter_prop(&mut self, node: &GetterProp) {
        if let PropName::Computed(computed) = &node.key
            && self.transform.is_dynamic(&computed.expr, self.opts)
        {
            self.found = true;
        }
    }

    fn visit_setter_prop(&mut self, node: &SetterProp) {
        if let PropName::Computed(computed) = &node.key
            && self.transform.is_dynamic(&computed.expr, self.opts)
        {
            self.found = true;
        }
    }

    fn visit_call_expr(&mut self, node: &CallExpr) {
        if self.opts.check_call_expressions {
            self.found = true;
            return;
        }
        use swc_core::ecma::visit::VisitWith;
        node.visit_children_with(self);
    }

    fn visit_opt_chain_expr(&mut self, node: &OptChainExpr) {
        let hit = match &*node.base {
            OptChainBase::Call(_) => self.opts.check_call_expressions,
            OptChainBase::Member(_) => self.opts.check_member,
        };
        if hit {
            self.found = true;
            return;
        }
        use swc_core::ecma::visit::VisitWith;
        node.visit_children_with(self);
    }

    fn visit_member_expr(&mut self, node: &MemberExpr) {
        if self.opts.check_member {
            self.found = true;
            return;
        }
        use swc_core::ecma::visit::VisitWith;
        node.visit_children_with(self);
    }

    /// An array or argument spread is a field on `ExprOrSpread` in SWC, not the `SpreadElement`
    /// node Babel visits, so it needs its own check.
    fn visit_expr_or_spread(&mut self, node: &ExprOrSpread) {
        if self.opts.check_member && node.spread.is_some() {
            self.found = true;
            return;
        }
        node.visit_children_with(self);
    }

    fn visit_spread_element(&mut self, node: &SpreadElement) {
        if self.opts.check_member {
            self.found = true;
            return;
        }
        use swc_core::ecma::visit::VisitWith;
        node.visit_children_with(self);
    }

    fn visit_bin_expr(&mut self, node: &BinExpr) {
        if self.opts.check_member && node.op == BinaryOp::In {
            self.found = true;
            return;
        }
        use swc_core::ecma::visit::VisitWith;
        node.visit_children_with(self);
    }

    fn visit_jsx_element(&mut self, node: &JSXElement) {
        let _ = node;
        if self.opts.check_tags {
            self.found = true;
        }
    }

    fn visit_jsx_fragment(&mut self, node: &JSXFragment) {
        let _ = node;
        if self.opts.check_tags {
            self.found = true;
        }
    }
}

// ---------------------------------------------------------------- static expressions

impl<C: Comments> Transform<C> {
    /// `getStaticExpression`: the literal text an expression container collapses to when it
    /// sits directly inside a native element.
    pub(crate) fn get_static_expression(
        &self,
        child: &JSXElementChild,
        parent_is_component: bool,
    ) -> Option<String> {
        let JSXElementChild::JSXExprContainer(container) = child else {
            return None;
        };
        self.static_expression_of(container, parent_is_component)
    }

    /// The same, for a container already in hand — taking the child would mean cloning it.
    pub(crate) fn static_expression_of(
        &self,
        container: &JSXExprContainer,
        parent_is_component: bool,
    ) -> Option<String> {
        if parent_is_component {
            return None;
        }
        let JSXExpr::Expr(expr) = &container.expr else {
            return None;
        };
        if matches!(&**expr, Expr::Seq(_)) {
            return None;
        }
        match evaluate(expr, &self.bindings)? {
            Value::Str(s) => Some(s),
            Value::Num(n) => Some(crate::eval::number_to_string(n)),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------- builders

pub(crate) fn ident(name: &str) -> Ident {
    Ident::new_no_ctxt(name.into(), DUMMY_SP)
}

pub(crate) fn is_logical(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::LogicalAnd | BinaryOp::LogicalOr | BinaryOp::NullishCoalescing
    )
}

/// `wrappedByText`: is this child sandwiched between text nodes with no element between?
pub(crate) fn wrapped_by_text(ids: &[Option<Ident>], texts: &[bool], start: usize) -> bool {
    let mut wrapped = false;
    for index in (0..start).rev() {
        if texts[index] {
            wrapped = true;
            break;
        }
        if ids[index].is_some() {
            return false;
        }
    }
    if !wrapped {
        return false;
    }
    for index in start + 1..texts.len() {
        if texts[index] {
            return true;
        }
        if ids[index].is_some() {
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_whitespace_the_way_babel_does() {
        assert_eq!(trim_whitespace("  a  b  "), " a b ");
        // A blank first line is dropped, later lines lose their indent, and the rest join.
        assert_eq!(trim_whitespace("\n  Hello\n  World\n"), "Hello World");
        assert_eq!(trim_whitespace("a\r\n  b"), "a b");
        // A non-breaking space is JS whitespace too.
        assert_eq!(trim_whitespace("a\u{a0}\u{a0}b"), "a b");
    }

    #[test]
    fn escapes_only_what_the_context_needs() {
        assert_eq!(escape_html("a<b&c\"d", false), "a&lt;b&amp;c\"d");
        assert_eq!(escape_html("a<b&c\"d", true), "a<b&amp;c&quot;d");
    }

    #[test]
    fn recognises_components_by_tag_shape() {
        assert!(is_component("Foo"));
        assert!(is_component("ns.Foo"));
        assert!(is_component("_foo"));
        assert!(!is_component("div"));
        assert!(!is_component("my-element"));
    }

    #[test]
    fn keeps_reserved_words_out_of_plain_property_keys() {
        assert!(is_valid_identifier("foo$_1"));
        assert!(!is_valid_identifier("class"));
        assert!(!is_valid_identifier("hyphen-ated"));
        assert!(!is_valid_identifier("1a"));
    }
}
