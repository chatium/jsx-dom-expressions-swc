use std::collections::HashSet;

use swc_core::common::comments::Comments;
use swc_core::common::{DUMMY_SP, Spanned};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use crate::template::{arrow_block, arrow_expr, call, double_bang, expr_stmt, str_lit};
use crate::utils::{
    DynamicOpts, escape_backticks, escape_html, filter_children, get_tag_name, is_component,
    is_logical, trim_whitespace,
};
use crate::{Generate, Transform};

#[derive(Debug, Clone)]
pub(crate) struct Dynamic {
    pub(crate) elem: Ident,
    pub(crate) key: String,
    pub(crate) value: Box<Expr>,
    pub(crate) is_svg: bool,
    pub(crate) is_ce: bool,
    pub(crate) tag_name: String,
}

#[derive(Default)]
pub(crate) struct Results {
    pub(crate) template: String,
    /// The SSR generator builds a list of static chunks with expressions interleaved.
    pub(crate) template_parts: Vec<String>,
    /// Boxed because every consumer hands these straight to the AST, which wants `Box<Expr>`.
    #[allow(clippy::vec_box)]
    pub(crate) template_values: Vec<Box<Expr>>,
    pub(crate) spread_element: bool,
    pub(crate) wont_escape: bool,
    pub(crate) declarations: Vec<VarDeclarator>,
    /// Statements for an element result; for a value result, one `Stmt::Expr` holding it.
    pub(crate) exprs: Vec<Stmt>,
    pub(crate) dynamics: Vec<Dynamic>,
    pub(crate) post_exprs: Vec<Stmt>,
    pub(crate) id: Option<Ident>,
    pub(crate) is_svg: bool,
    pub(crate) has_custom_element: bool,
    pub(crate) tag_name: Option<String>,
    pub(crate) text: bool,
    pub(crate) dynamic: bool,
    pub(crate) skip_template: bool,
    pub(crate) has_hydratable_event: bool,
    pub(crate) to_be_closed: Option<HashSet<String>>,
    pub(crate) decl: Option<Stmt>,
}

impl Results {
    pub(crate) fn from_expr(expr: Box<Expr>) -> Self {
        Self {
            exprs: vec![expr_stmt(expr)],
            ..Default::default()
        }
    }

    pub(crate) fn into_first_expr(mut self) -> Box<Expr> {
        match self.exprs.remove(0) {
            Stmt::Expr(stmt) => stmt.expr,
            other => unreachable!("value result held a statement: {other:?}"),
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct Info {
    pub(crate) top_level: bool,
    pub(crate) last_element: bool,
    pub(crate) skip_id: bool,
    pub(crate) component_child: bool,
    pub(crate) fragment_child: bool,
    /// Inside `<script>`, `<style>` or an `innerHTML`, SSR text is emitted verbatim.
    pub(crate) do_not_escape: bool,
    pub(crate) to_be_closed: Option<HashSet<String>>,
    /// Whether the node sits directly inside a native element, which is what
    /// `getStaticExpression` checks through `path.parent`.
    pub(crate) parent_is_native: bool,
}

pub(crate) enum Condition {
    Expr(Box<Expr>),
    /// `[const _c$ = …, () => expr]` — the caller hoists the declaration.
    Hoisted(Stmt, Box<Expr>),
}

impl<C: Comments> Transform<C> {
    /// `transformJSX`: the entry point for a top-level JSX element or fragment.
    pub(crate) fn transform_jsx(&mut self, mut expr: Expr) -> Expr {
        // Babel's AST has no parenthesised-expression node, and every `isConditionalExpression`
        // style check downstream assumes that. Dropping them inside the JSX subtree keeps those
        // checks honest; SWC's printer re-adds whatever precedence needs.
        expr.visit_mut_with(&mut RemoveParens);
        self.hoist_container_comments(&expr);
        let this_id = self.transform_this(&mut expr);
        let (child, info) = match expr {
            Expr::JSXElement(el) => (
                JSXElementChild::JSXElement(el),
                Info {
                    top_level: true,
                    last_element: true,
                    ..Default::default()
                },
            ),
            Expr::JSXFragment(fragment) => {
                (JSXElementChild::JSXFragment(fragment), Info::default())
            }
            other => return other,
        };
        let result = self
            .transform_node(child, info)
            .expect("a top-level element always produces a result");
        let out = self.create_template_for(result, false);
        if let Some(id) = this_id {
            self.pending.push(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                span: DUMMY_SP,
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(id.into()),
                    init: Some(Box::new(Expr::This(ThisExpr { span: DUMMY_SP }))),
                    definite: false,
                }],
                ..Default::default()
            }))));
        }
        *out
    }

    /// SWC attaches `{/*@once*/ expr}` as a comment trailing the `{`, where Babel has it
    /// leading the expression. Moving it makes the static-marker check see what Babel sees.
    fn hoist_container_comments(&self, expr: &Expr) {
        struct Walk<'a, C: Comments>(&'a C);
        impl<C: Comments> swc_core::ecma::visit::Visit for Walk<'_, C> {
            fn visit_jsx_expr_container(&mut self, node: &JSXExprContainer) {
                if let JSXExpr::Expr(inner) = &node.expr {
                    let after_brace = node.span.lo + swc_core::common::BytePos(1);
                    if let Some(trailing) = self.0.take_trailing(after_brace) {
                        self.0.add_leading_comments(inner.span().lo, trailing);
                    }
                }
                node.visit_children_with(self);
            }
        }
        use swc_core::ecma::visit::VisitWith;
        expr.visit_with(&mut Walk(&self.comments));
    }

    /// `transformThis`: lift `this` out of the JSX so the generated closures keep meaning it.
    fn transform_this(&mut self, expr: &mut Expr) -> Option<Ident> {
        struct Replacer {
            id: Option<Ident>,
        }
        impl VisitMut for Replacer {
            fn visit_mut_expr(&mut self, expr: &mut Expr) {
                if matches!(expr, Expr::This(_)) {
                    let id = self.id.clone().expect("prepared before the walk");
                    *expr = Expr::Ident(id);
                    return;
                }
                expr.visit_mut_children_with(self);
            }
            fn visit_mut_function(&mut self, _: &mut Function) {}
        }
        if !has_this(expr) {
            return None;
        }
        let id = self.uid.generate("self$");
        expr.visit_mut_with(&mut Replacer {
            id: Some(id.clone()),
        });
        Some(id)
    }

    pub(crate) fn transform_node(&mut self, node: JSXElementChild, info: Info) -> Option<Results> {
        match node {
            JSXElementChild::JSXElement(el) => Some(self.transform_element_or_component(*el, info)),
            JSXElementChild::JSXFragment(fragment) => {
                let mut results = Results::default();
                self.transform_fragment_children(fragment.children, &mut results);
                Some(results)
            }
            JSXElementChild::JSXText(text) => self.text_result(trim_whitespace(&text.raw), &info),
            JSXElementChild::JSXExprContainer(container) => {
                if let Some(value) = self.get_static_expression(
                    &JSXElementChild::JSXExprContainer(container.clone()),
                    !info.parent_is_native,
                ) {
                    let text = if info.do_not_escape {
                        value
                    } else {
                        escape_html(&value, false)
                    };
                    return self.text_result(text, &info);
                }
                let JSXExpr::Expr(expression) = container.expr else {
                    return None;
                };
                self.expression_result(expression, &info)
            }
            JSXElementChild::JSXSpreadChild(spread) => {
                if !self.is_dynamic(
                    &spread.expr,
                    DynamicOpts::new().member().native(!info.component_child),
                ) {
                    return Some(Results::from_expr(spread.expr));
                }
                Some(Results {
                    dynamic: true,
                    ..Results::from_expr(arrow_expr(Vec::new(), spread.expr))
                })
            }
        }
    }

    fn text_result(&mut self, text: String, info: &Info) -> Option<Results> {
        if text.is_empty() {
            return None;
        }
        if self.config.generate == Generate::Ssr {
            return Some(Results {
                template_parts: vec![text],
                text: true,
                ..Default::default()
            });
        }
        Some(Results {
            template: escape_backticks(&text),
            text: true,
            id: (!info.skip_id).then(|| self.uid.generate("el$")),
            ..Default::default()
        })
    }

    fn expression_result(&mut self, expression: Box<Expr>, info: &Info) -> Option<Results> {
        let mut opts = DynamicOpts::new().member().native(!info.component_child);
        opts.check_tags = info.component_child;
        if !self.is_dynamic(&expression, opts) {
            return Some(Results::from_expr(expression));
        }
        let ssr = self.config.generate == Generate::Ssr;
        let wrap_conditional = self.config.wrap_conditionals
            && !ssr
            && (matches!(&*expression, Expr::Bin(bin) if is_logical(bin.op))
                || matches!(&*expression, Expr::Cond(_)));
        let condition = if wrap_conditional {
            self.transform_condition(expression, info.component_child, false)
        } else if !info.component_child
            && (!ssr || info.fragment_child)
            && is_zero_arg_call(&expression)
        {
            let Expr::Call(call) = *expression else {
                unreachable!()
            };
            let Callee::Expr(callee) = call.callee else {
                unreachable!()
            };
            Condition::Expr(callee)
        } else {
            Condition::Expr(arrow_expr(Vec::new(), expression))
        };
        let expr = match condition {
            Condition::Expr(expr) => expr,
            Condition::Hoisted(decl, arrow) => call(
                arrow_block(
                    Vec::new(),
                    vec![
                        decl,
                        Stmt::Return(ReturnStmt {
                            span: DUMMY_SP,
                            arg: Some(arrow),
                        }),
                    ],
                ),
                Vec::new(),
            ),
        };
        Some(Results {
            dynamic: true,
            ..Results::from_expr(expr)
        })
    }

    fn transform_element_or_component(&mut self, el: JSXElement, info: Info) -> Results {
        let tag_name = get_tag_name(&el);
        if is_component(&tag_name) {
            self.transform_component(el)
        } else if self.config.generate == Generate::Ssr {
            self.transform_element_ssr(el, info)
        } else {
            self.transform_element(el, info)
        }
    }

    /// `getCreateTemplate`. With no per-tag `renderers`, the configured generator decides.
    pub(crate) fn create_template_for(&mut self, result: Results, wrap: bool) -> Box<Expr> {
        if self.config.generate == Generate::Ssr {
            self.create_template_ssr(result)
        } else {
            self.create_template(result, wrap)
        }
    }

    /// `transformFragmentChildren`.
    pub(crate) fn transform_fragment_children(
        &mut self,
        children: Vec<JSXElementChild>,
        results: &mut Results,
    ) {
        let mut child_nodes = Vec::new();
        for child in filter_children(children) {
            if let JSXElementChild::JSXText(text) = &child {
                let value = decode_html_entities(&trim_whitespace(&text.raw));
                if !value.is_empty() {
                    child_nodes.push(Box::new(str_lit(&value)));
                }
                continue;
            }
            let info = Info {
                top_level: true,
                fragment_child: true,
                last_element: true,
                ..Default::default()
            };
            if let Some(result) = self.transform_node(child, info) {
                let expr = self.create_template_for(result, true);
                child_nodes.push(expr);
            }
        }
        let expr = if child_nodes.len() == 1 {
            child_nodes.remove(0)
        } else {
            Box::new(Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: child_nodes
                    .into_iter()
                    .map(|expr| Some(ExprOrSpread { spread: None, expr }))
                    .collect(),
            }))
        };
        results.exprs.push(expr_stmt(expr));
    }

    /// `transformCondition`.
    pub(crate) fn transform_condition(
        &mut self,
        mut expr: Box<Expr>,
        inline: bool,
        deep: bool,
    ) -> Condition {
        // Registered unconditionally, exactly as upstream does — the import appears even when
        // the wrapper is not used.
        let memo = self.register_import(&self.config.memo_wrapper.clone());

        let mut cond: Option<Box<Expr>> = None;
        let mut id: Option<Box<Expr>> = None;
        let mut hoist_id: Option<Ident> = None;

        let make_id = |this: &mut Self, cond: &Expr, hoist_id: &mut Option<Ident>| -> Box<Expr> {
            if inline {
                call(
                    Box::new(Expr::Ident(memo.clone())),
                    vec![crate::template::arg(arrow_expr(
                        Vec::new(),
                        Box::new(cond.clone()),
                    ))],
                )
            } else {
                let uid = this.uid.generate("_c$");
                *hoist_id = Some(uid.clone());
                Box::new(Expr::Ident(uid))
            }
        };

        match &mut *expr {
            Expr::Cond(conditional)
                if self.is_dynamic(&conditional.cons, DynamicOpts::new().tags())
                    || self.is_dynamic(&conditional.alt, DynamicOpts::new().tags()) =>
            {
                if self.is_dynamic(&conditional.test, DynamicOpts::new().member()) {
                    let mut test = conditional.test.clone();
                    if !is_binary(&test) {
                        test = double_bang(test);
                    }
                    let generated = make_id(self, &test, &mut hoist_id);
                    cond = Some(test);
                    conditional.test = call(generated.clone(), Vec::new());
                    id = Some(generated);
                    if matches!(&*conditional.cons, Expr::Cond(_))
                        || matches!(&*conditional.cons, Expr::Bin(bin) if is_logical(bin.op))
                    {
                        let inner = std::mem::replace(
                            &mut conditional.cons,
                            Box::new(Expr::Invalid(Invalid { span: DUMMY_SP })),
                        );
                        conditional.cons = self.transform_condition_deep(inner, inline);
                    }
                    if matches!(&*conditional.alt, Expr::Cond(_))
                        || matches!(&*conditional.alt, Expr::Bin(bin) if is_logical(bin.op))
                    {
                        let inner = std::mem::replace(
                            &mut conditional.alt,
                            Box::new(Expr::Invalid(Invalid { span: DUMMY_SP })),
                        );
                        conditional.alt = self.transform_condition_deep(inner, inline);
                    }
                }
            }
            Expr::Bin(bin) if is_logical(bin.op) => {
                let target = descend_to_and(bin);
                let applies = target.op == op!("&&")
                    && self.is_dynamic(&target.right, DynamicOpts::new().tags())
                    && self.is_dynamic(&target.left, DynamicOpts::new().member());
                if applies {
                    let mut left = target.left.clone();
                    if !is_binary(&left) {
                        left = double_bang(left);
                    }
                    let generated = make_id(self, &left, &mut hoist_id);
                    cond = Some(left);
                    target.left = call(generated.clone(), Vec::new());
                    id = Some(generated);
                }
            }
            _ => {}
        }

        let _ = id;
        match (cond, hoist_id) {
            (Some(cond), Some(hoist_id)) => {
                let init = if self.config.memo_wrapper.is_empty() {
                    arrow_expr(Vec::new(), cond)
                } else {
                    call(
                        Box::new(Expr::Ident(memo)),
                        vec![crate::template::arg(arrow_expr(Vec::new(), cond))],
                    )
                };
                let decl = Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Const,
                    declare: false,
                    decls: vec![VarDeclarator {
                        span: DUMMY_SP,
                        name: Pat::Ident(hoist_id.into()),
                        init: Some(init),
                        definite: false,
                    }],
                    ..Default::default()
                })));
                let arrow = arrow_expr(Vec::new(), expr);
                if deep {
                    Condition::Expr(call(
                        arrow_block(
                            Vec::new(),
                            vec![
                                decl,
                                Stmt::Return(ReturnStmt {
                                    span: DUMMY_SP,
                                    arg: Some(arrow),
                                }),
                            ],
                        ),
                        Vec::new(),
                    ))
                } else {
                    Condition::Hoisted(decl, arrow)
                }
            }
            _ => Condition::Expr(if deep {
                expr
            } else {
                arrow_expr(Vec::new(), expr)
            }),
        }
    }

    fn transform_condition_deep(&mut self, expr: Box<Expr>, inline: bool) -> Box<Expr> {
        match self.transform_condition(expr, inline, true) {
            Condition::Expr(expr) => expr,
            Condition::Hoisted(..) => unreachable!("deep never hoists"),
        }
    }
}

struct RemoveParens;

impl VisitMut for RemoveParens {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);
        if let Expr::Paren(paren) = expr {
            *expr = *std::mem::replace(
                &mut paren.expr,
                Box::new(Expr::Invalid(Invalid { span: DUMMY_SP })),
            );
        }
    }

    /// SWC's JSX string lexer emits a line terminator as the raw bytes *plus* a normalised
    /// LF, so a CRLF arrives as `\r\n\n`. Babel keeps the source terminator as written
    /// (`jsxReadNewLine` with `normalizeCRLF` false), and SSR puts the value in a string
    /// literal where the difference survives, so undo the duplication rather than fold it.
    fn visit_mut_jsx_attr_value(&mut self, value: &mut JSXAttrValue) {
        value.visit_mut_children_with(self);
        if let JSXAttrValue::Str(text) = value {
            // A JSX string may span lines and hold raw quotes, which are legal there and not in
            // a JS string literal. Babel rebuilds the literal so its printer escapes it; keeping
            // SWC's `raw` would emit the source bytes verbatim and produce invalid JS.
            text.raw = None;
            let raw = text.value.to_atom_lossy();
            if raw.contains('\r') {
                let mut out = String::with_capacity(raw.len());
                let mut chars = raw.chars().peekable();
                while let Some(c) = chars.next() {
                    out.push(c);
                    if c != '\r' || chars.peek() != Some(&'\n') {
                        continue;
                    }
                    chars.next();
                    if chars.peek() == Some(&'\n') {
                        // A real CRLF: SWC added the extra LF.
                        chars.next();
                        out.push('\n');
                    }
                    // Otherwise this was a lone CR and the LF is SWC's, so drop it.
                }
                text.value = out.into();
            }
        }
    }
}

/// `while (nextPath.node.operator !== "&&" && isLogicalExpression(nextPath.node.left))`.
fn descend_to_and(bin: &mut BinExpr) -> &mut BinExpr {
    if bin.op != op!("&&") && matches!(&*bin.left, Expr::Bin(inner) if is_logical(inner.op)) {
        let Expr::Bin(inner) = &mut *bin.left else {
            unreachable!()
        };
        return descend_to_and(inner);
    }
    bin
}

/// Babel keeps `LogicalExpression` separate from `BinaryExpression`; SWC folds both into
/// `BinExpr`, so `isBinaryExpression` has to exclude the logical operators explicitly.
pub(crate) fn is_binary(expr: &Expr) -> bool {
    matches!(expr, Expr::Bin(bin) if !is_logical(bin.op))
}

fn is_zero_arg_call(expr: &Expr) -> bool {
    matches!(expr, Expr::Call(call)
        if call.args.is_empty()
            && matches!(&call.callee, Callee::Expr(callee) if !matches!(&**callee, Expr::Member(_))))
}

fn has_this(expr: &Expr) -> bool {
    use swc_core::ecma::visit::{Visit, VisitWith};
    struct Find(bool);
    impl Visit for Find {
        fn visit_this_expr(&mut self, _: &ThisExpr) {
            self.0 = true;
        }
        fn visit_function(&mut self, _: &Function) {}
    }
    let mut find = Find(false);
    expr.visit_with(&mut find);
    find.0
}

/// `html-entities`' `decode`, limited to what JSX text can carry.
pub(crate) fn decode_html_entities(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }
    crate::entities::decode(text)
}

#[allow(dead_code)]
fn unused(span: swc_core::common::Span) -> swc_core::common::Span {
    span.span()
}
