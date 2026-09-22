use std::collections::HashSet;

use swc_core::common::comments::Comments;
use swc_core::common::{DUMMY_SP, Spanned};
use swc_core::ecma::ast::*;

use crate::Transform;
use crate::constants::{
    ALIASES, ALWAYS_CLOSE, BLOCK_ELEMENTS, CHILD_PROPERTIES, DELEGATED_EVENTS, INLINE_ELEMENTS,
    PROPERTIES, SVG_ELEMENTS, SVG_NAMESPACE, VOID_ELEMENTS, get_prop_alias,
};
use crate::eval::{Value, evaluate};
use crate::template::{
    arg, arrow_expr, bool_arg, call, double_bang, expr_stmt, expr_to_assign_target, member, str_lit,
};
use crate::transform::{Condition, Dynamic, Info, Results};
use crate::utils::{
    DynamicOpts, attr_key, can_native_spread, check_length, convert_jsx_identifier,
    escape_backticks, escape_html, filter_children, filter_children_ref, get_tag_name, ident,
    is_component, is_empty_expr_container, is_logical, is_reserved_namespace, is_valid_identifier,
    to_event_name, to_property_name, trim_whitespace, wrapped_by_text,
};

#[derive(Default)]
pub(crate) struct SetAttrOpts {
    pub(crate) is_svg: bool,
    pub(crate) is_ce: bool,
    pub(crate) tag_name: String,
    pub(crate) dynamic: bool,
    pub(crate) prev_id: Option<Box<Expr>>,
}

impl<C: Comments> Transform<C> {
    pub(crate) fn transform_element(&mut self, mut el: JSXElement, info: Info) -> Results {
        let tag_name = get_tag_name(&el);
        let wrap_svg = info.top_level && tag_name != "svg" && SVG_ELEMENTS.contains(&&*tag_name);
        let void_tag = VOID_ELEMENTS.contains(&&*tag_name);
        let is_custom_element = tag_name.contains('-');
        let mut results = Results {
            template: format!("<{tag_name}"),
            is_svg: wrap_svg,
            has_custom_element: is_custom_element,
            tag_name: Some(tag_name.clone()),
            ..Default::default()
        };
        if self.config.hydratable && matches!(tag_name.as_str(), "html" | "head" | "body") {
            results.skip_template = true;
            if tag_name == "head" && info.top_level {
                let create_component = self.register_import("createComponent");
                let no_hydration = self.register_import("NoHydration");
                results.exprs.push(expr_stmt(call(
                    Box::new(Expr::Ident(create_component)),
                    vec![
                        arg(Box::new(Expr::Ident(no_hydration))),
                        arg(Box::new(Expr::Object(ObjectLit {
                            span: DUMMY_SP,
                            props: Vec::new(),
                        }))),
                    ],
                )));
                return results;
            }
        }
        if wrap_svg {
            results.template = format!("<svg>{}", results.template);
        }
        if !info.skip_id {
            results.id = Some(self.uid.generate("el$"));
        }
        self.transform_attributes(&mut el, &mut results);
        if self.config.context_to_custom_elements && (tag_name == "slot" || is_custom_element) {
            let owner = self.register_import("getOwner");
            let id = results
                .id
                .clone()
                .expect("a context element always has an id");
            results
                .exprs
                .push(expr_stmt(Box::new(Expr::Assign(AssignExpr {
                    span: DUMMY_SP,
                    op: op!("="),
                    left: expr_to_assign_target(member(Box::new(Expr::Ident(id)), "_$owner")),
                    right: call(Box::new(Expr::Ident(owner)), Vec::new()),
                }))));
        }
        results.template.push('>');
        if !void_tag {
            let to_be_closed = !info.last_element
                || info.to_be_closed.as_ref().is_some_and(|set| {
                    !self.config.omit_nested_closing_tags || set.contains(&tag_name)
                });
            if to_be_closed {
                let mut set: HashSet<String> = info
                    .to_be_closed
                    .clone()
                    .unwrap_or_else(|| ALWAYS_CLOSE.iter().map(|tag| tag.to_string()).collect());
                set.insert(tag_name.clone());
                if INLINE_ELEMENTS.contains(&&*tag_name) {
                    set.extend(BLOCK_ELEMENTS.iter().map(|tag| tag.to_string()));
                }
                results.to_be_closed = Some(set);
            } else {
                results.to_be_closed = info.to_be_closed.clone();
            }
            let children = std::mem::take(&mut el.children);
            self.transform_children(children, &tag_name, &mut results);
            if to_be_closed {
                results.template.push_str(&format!("</{tag_name}>"));
            }
        }
        if info.top_level && self.config.hydratable && results.has_hydratable_event {
            let run = self.register_import("runHydrationEvents");
            results
                .post_exprs
                .push(expr_stmt(call(Box::new(Expr::Ident(run)), Vec::new())));
        }
        if wrap_svg {
            results.template.push_str("</svg>");
        }
        results
    }

    pub(crate) fn set_attr(
        &mut self,
        elem: Box<Expr>,
        name: &str,
        value: Box<Expr>,
        opts: SetAttrOpts,
    ) -> Box<Expr> {
        let mut name = name.to_string();
        let mut namespace = None;
        if let Some((ns, rest)) = name.split_once(':')
            && !rest.is_empty()
            && crate::constants::RESERVED_NAMESPACES.contains(&ns)
        {
            namespace = Some(ns.to_string());
            name = rest.to_string();
        }

        if namespace.as_deref() == Some("style") {
            let style = member(elem.clone(), "style");
            if matches!(&*value, Expr::Lit(Lit::Str(_))) {
                return call(
                    member(style, "setProperty"),
                    vec![arg(Box::new(str_lit(&name))), arg(value)],
                );
            }
            if matches!(&*value, Expr::Lit(Lit::Null(_)))
                || matches!(&*value, Expr::Ident(id) if id.sym == *"undefined")
            {
                return call(
                    member(style, "removeProperty"),
                    vec![arg(Box::new(str_lit(&name)))],
                );
            }
            return Box::new(Expr::Cond(CondExpr {
                span: DUMMY_SP,
                test: Box::new(Expr::Bin(BinExpr {
                    span: DUMMY_SP,
                    op: op!("!="),
                    left: value.clone(),
                    right: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
                })),
                cons: call(
                    member(style.clone(), "setProperty"),
                    vec![
                        arg(Box::new(str_lit(&name))),
                        arg(opts.prev_id.clone().unwrap_or(value)),
                    ],
                ),
                alt: call(
                    member(style, "removeProperty"),
                    vec![arg(Box::new(str_lit(&name)))],
                ),
            }));
        }

        if namespace.as_deref() == Some("class") {
            return call(
                member(member(elem, "classList"), "toggle"),
                vec![
                    arg(Box::new(str_lit(&name))),
                    arg(if opts.dynamic {
                        value
                    } else {
                        double_bang(value)
                    }),
                ],
            );
        }

        if name == "style" {
            let style = self.register_import("style");
            let mut args = vec![arg(elem), arg(value)];
            args.extend(opts.prev_id.map(arg));
            return call(Box::new(Expr::Ident(style)), args);
        }

        if !opts.is_svg && name == "class" {
            let class_name = self.register_import("className");
            return call(
                Box::new(Expr::Ident(class_name)),
                vec![arg(elem), arg(value)],
            );
        }

        if name == "classList" {
            let class_list = self.register_import("classList");
            let mut args = vec![arg(elem), arg(value)];
            args.extend(opts.prev_id.map(arg));
            return call(Box::new(Expr::Ident(class_list)), args);
        }

        if self.config.hydratable && name == "innerHTML" {
            let inner_html = self.register_import("innerHTML");
            return call(
                Box::new(Expr::Ident(inner_html)),
                vec![arg(elem), arg(value)],
            );
        }

        if opts.dynamic && name == "textContent" {
            return Box::new(Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: op!("="),
                left: expr_to_assign_target(member(elem, "data")),
                right: value,
            }));
        }

        let is_child_prop = CHILD_PROPERTIES.contains(&&*name);
        let is_prop = PROPERTIES.contains(&&*name);
        let alias = get_prop_alias(&name, &opts.tag_name.to_uppercase());
        if namespace.as_deref() != Some("attr")
            && (is_child_prop
                || (!opts.is_svg && is_prop)
                || opts.is_ce
                || namespace.as_deref() == Some("prop"))
        {
            if opts.is_ce && !is_child_prop && !is_prop && namespace.as_deref() != Some("prop") {
                name = to_property_name(&name);
            }
            return Box::new(Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: op!("="),
                left: expr_to_assign_target(member(elem, alias.unwrap_or(&name))),
                right: value,
            }));
        }

        let is_namespaced = name.contains(':');
        if let Some((_, aliased)) = ALIASES.iter().find(|(from, _)| *from == name) {
            name = aliased.to_string();
        }
        if !opts.is_svg {
            name = name.to_lowercase();
        }
        let ns = is_namespaced
            .then(|| {
                SVG_NAMESPACE
                    .iter()
                    .find(|(prefix, _)| *prefix == name.split(':').next().unwrap())
                    .map(|(_, uri)| *uri)
            })
            .flatten();
        match ns {
            Some(uri) => {
                let set_attribute_ns = self.register_import("setAttributeNS");
                call(
                    Box::new(Expr::Ident(set_attribute_ns)),
                    vec![
                        arg(elem),
                        arg(Box::new(str_lit(uri))),
                        arg(Box::new(str_lit(&name))),
                        arg(value),
                    ],
                )
            }
            None => {
                let set_attribute = self.register_import("setAttribute");
                call(
                    Box::new(Expr::Ident(set_attribute)),
                    vec![arg(elem), arg(Box::new(str_lit(&name))), arg(value)],
                )
            }
        }
    }

    fn detect_resolvable_event_handler(&self, handler: &Expr) -> bool {
        let mut current = handler.clone();
        while let Expr::Ident(id) = &current {
            match self.bindings.get(id) {
                Some(binding) if binding.is_function_decl => return true,
                Some(binding) => match &binding.init {
                    Some(init) => current = (**init).clone(),
                    None => return false,
                },
                None => return false,
            }
        }
        matches!(current, Expr::Fn(_) | Expr::Arrow(_))
    }
}

// ---------------------------------------------------------------- helpers shared with element.rs

impl<C: Comments> Transform<C> {
    /// `p.value.leadingComments = leading`: an `@once` on a style or classList object applies
    /// to every property it is split into.
    fn copy_leading(&self, from: swc_core::common::BytePos, to: swc_core::common::BytePos) {
        if from == to {
            return;
        }
        if let Some(comments) = self.comments.get_leading(from) {
            self.comments.add_leading_comments(to, comments);
        }
    }
}

pub(crate) fn js_splice_remove<T>(items: &mut Vec<T>, index: i64) {
    let len = items.len() as i64;
    let start = if index < 0 {
        (len + index).max(0)
    } else {
        index.min(len)
    };
    if start < len {
        items.remove(start as usize);
    }
}

// ---------------------------------------------------------------- attributes

impl<C: Comments> Transform<C> {
    fn transform_attributes(&mut self, el: &mut JSXElement, results: &mut Results) {
        let elem = results.id.clone();
        let tag_name = get_tag_name(el);
        let is_svg = SVG_ELEMENTS.contains(&&*tag_name);
        let is_ce = tag_name.contains('-');
        let has_children = !el.children.is_empty();
        let mut children: Option<JSXElementChild> = None;
        let mut spread_expr = None;

        if el
            .opening
            .attrs
            .iter()
            .any(|attr| matches!(attr, JSXAttrOrSpread::SpreadElement(_)))
        {
            let elem = elem.clone().expect("a spread element always has an id");
            let (kept, expr) = self.process_spreads(
                std::mem::take(&mut el.opening.attrs),
                Box::new(Expr::Ident(elem)),
                is_svg,
                has_children,
            );
            el.opening.attrs = kept;
            spread_expr = Some(expr);
            // Can't be known at compile time, so the compiled output has to assume it.
            results.has_hydratable_event = true;
        }

        self.preprocess_style(&mut el.opening.attrs);
        self.preprocess_class_list(&mut el.opening.attrs);
        combine_class_attributes(&mut el.opening.attrs);

        for attribute in std::mem::take(&mut el.opening.attrs) {
            let JSXAttrOrSpread::JSXAttr(node) = attribute else {
                continue;
            };
            let mut value = node.value.clone();
            let key = attr_key(&node.name);
            let reserved_namespace = is_reserved_namespace(&node.name);

            if let Some(JSXAttrValue::JSXExprContainer(container)) = &value
                && !key.starts_with("use:")
                && let JSXExpr::Expr(expr) = &container.expr
                && let Some(text) = evaluate(expr, &self.bindings)
                    .as_ref()
                    .and_then(Value::as_static_text)
            {
                value = Some(JSXAttrValue::Str(Str {
                    span: DUMMY_SP,
                    value: text.into(),
                    raw: None,
                }));
            }

            if reserved_namespace && !matches!(value, Some(JSXAttrValue::JSXExprContainer(_))) {
                value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                    span: DUMMY_SP,
                    expr: match value {
                        Some(JSXAttrValue::Str(s)) => {
                            JSXExpr::Expr(Box::new(Expr::Lit(Lit::Str(s))))
                        }
                        Some(JSXAttrValue::JSXElement(el)) => {
                            JSXExpr::Expr(Box::new(Expr::JSXElement(el)))
                        }
                        Some(JSXAttrValue::JSXFragment(fragment)) => {
                            JSXExpr::Expr(Box::new(Expr::JSXFragment(fragment)))
                        }
                        _ => JSXExpr::JSXEmptyExpr(JSXEmptyExpr { span: DUMMY_SP }),
                    },
                }));
            }

            let container_expr = match &value {
                Some(JSXAttrValue::JSXExprContainer(container)) => Some(container.expr.clone()),
                _ => None,
            };
            let takes_expression_path = container_expr.as_ref().is_some_and(|expr| {
                reserved_namespace
                    || !matches!(
                        expr,
                        JSXExpr::Expr(inner)
                            if matches!(&**inner, Expr::Lit(Lit::Str(_)) | Expr::Lit(Lit::Num(_)))
                    )
            });

            if takes_expression_path {
                let expression = match container_expr.unwrap() {
                    JSXExpr::Expr(expr) => Some(expr),
                    JSXExpr::JSXEmptyExpr(_) => None,
                };
                self.transform_expression_attribute(
                    &node,
                    &key,
                    expression,
                    elem.as_ref(),
                    &tag_name,
                    is_svg,
                    is_ce,
                    results,
                    &mut children,
                );
            } else {
                if self.config.hydratable && key == "$ServerOnly" {
                    results.skip_template = true;
                    continue;
                }
                let mut key = key;
                let mut value = match value {
                    Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        expr: JSXExpr::Expr(expr),
                        ..
                    })) => Some(*expr),
                    Some(JSXAttrValue::Str(s)) => Some(Expr::Lit(Lit::Str(s))),
                    _ => None,
                };
                if let Some((_, aliased)) = ALIASES.iter().find(|(from, _)| *from == key) {
                    key = aliased.to_string();
                }
                if value.is_some() && CHILD_PROPERTIES.contains(&&*key) {
                    let elem = elem.clone().expect("a child property needs an id");
                    let expr = self.set_attr(
                        Box::new(Expr::Ident(elem)),
                        &key,
                        Box::new(value.take().unwrap()),
                        SetAttrOpts {
                            is_svg,
                            is_ce,
                            tag_name: tag_name.clone(),
                            ..Default::default()
                        },
                    );
                    results.exprs.push(expr_stmt(expr));
                } else {
                    if !is_svg {
                        key = key.to_lowercase();
                    }
                    results.template.push(' ');
                    results.template.push_str(&key);
                    let Some(Expr::Lit(Lit::Str(text))) = value else {
                        continue;
                    };
                    let mut text = text.value.to_atom_lossy().to_string();
                    if key == "style" || key == "class" {
                        text = trim_whitespace(&text);
                        if key == "style" {
                            text = text.replace("; ", ";").replace(": ", ":");
                        }
                    }
                    results.template.push_str(&format!(
                        "=\"{}\"",
                        escape_backticks(&escape_html(&text, true))
                    ));
                }
            }
        }

        if !has_children && let Some(child) = children {
            el.children.push(child);
        }
        if let Some(expr) = spread_expr {
            results.exprs.push(expr);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn transform_expression_attribute(
        &mut self,
        node: &JSXAttr,
        key: &str,
        expression: Option<Box<Expr>>,
        elem: Option<&Ident>,
        tag_name: &str,
        is_svg: bool,
        is_ce: bool,
        results: &mut Results,
        children: &mut Option<JSXElementChild>,
    ) {
        let elem_expr = || Box::new(Expr::Ident(elem.cloned().expect("attributes need an id")));

        if key == "ref" {
            let Some(expression) = expression else { return };
            let expression = strip_ts_wrappers(expression);
            let is_const_fn = matches!(&*expression, Expr::Ident(id)
                if self.bindings.get(id).is_some_and(|binding| binding.is_const));
            // Upstream registers `use` inside each branch, after the uid it needs. Registering
            // it up front would import it for a ref that matches no branch at all, and would
            // shift the order the generated imports print in.
            if !is_const_fn && is_lval(&expression) {
                let ref_id = self.uid.generate("_ref$");
                let use_fn = self.register_import("use");
                let mut prefix = vec![
                    Stmt::Decl(Decl::Var(Box::new(VarDecl {
                        span: DUMMY_SP,
                        kind: VarDeclKind::Const,
                        declare: false,
                        decls: vec![VarDeclarator {
                            span: DUMMY_SP,
                            name: Pat::Ident(ref_id.clone().into()),
                            init: Some(expression.clone()),
                            definite: false,
                        }],
                        ..Default::default()
                    }))),
                    expr_stmt(Box::new(Expr::Cond(CondExpr {
                        span: DUMMY_SP,
                        test: typeof_is_function(&ref_id),
                        cons: call(
                            Box::new(Expr::Ident(use_fn)),
                            vec![arg(Box::new(Expr::Ident(ref_id))), arg(elem_expr())],
                        ),
                        alt: Box::new(Expr::Assign(AssignExpr {
                            span: DUMMY_SP,
                            op: op!("="),
                            left: expr_to_assign_target(expression),
                            right: elem_expr(),
                        })),
                    }))),
                ];
                prefix.append(&mut results.exprs);
                results.exprs = prefix;
            } else if is_const_fn || matches!(&*expression, Expr::Fn(_) | Expr::Arrow(_)) {
                let use_fn = self.register_import("use");
                let stmt = expr_stmt(call(
                    Box::new(Expr::Ident(use_fn)),
                    vec![arg(expression), arg(elem_expr())],
                ));
                results.exprs.insert(0, stmt);
            } else if matches!(&*expression, Expr::Call(_)) {
                let ref_id = self.uid.generate("_ref$");
                let use_fn = self.register_import("use");
                let mut prefix = vec![
                    Stmt::Decl(Decl::Var(Box::new(VarDecl {
                        span: DUMMY_SP,
                        kind: VarDeclKind::Const,
                        declare: false,
                        decls: vec![VarDeclarator {
                            span: DUMMY_SP,
                            name: Pat::Ident(ref_id.clone().into()),
                            init: Some(expression),
                            definite: false,
                        }],
                        ..Default::default()
                    }))),
                    expr_stmt(Box::new(Expr::Bin(BinExpr {
                        span: DUMMY_SP,
                        op: op!("&&"),
                        left: typeof_is_function(&ref_id),
                        right: call(
                            Box::new(Expr::Ident(use_fn)),
                            vec![arg(Box::new(Expr::Ident(ref_id))), arg(elem_expr())],
                        ),
                    }))),
                ];
                prefix.append(&mut results.exprs);
                results.exprs = prefix;
            }
            return;
        }

        if let Some(directive) = key.strip_prefix("use:") {
            let use_fn = self.register_import("use");
            let body = expression.unwrap_or_else(|| {
                Box::new(Expr::Lit(Lit::Bool(Bool {
                    span: DUMMY_SP,
                    value: true,
                })))
            });
            let stmt = expr_stmt(call(
                Box::new(Expr::Ident(use_fn)),
                vec![
                    arg(Box::new(Expr::Ident(ident(directive)))),
                    arg(elem_expr()),
                    arg(arrow_expr(Vec::new(), body)),
                ],
            ));
            results.exprs.insert(0, stmt);
            return;
        }

        if key == "children" {
            if let Some(expression) = expression {
                *children = Some(JSXElementChild::JSXExprContainer(JSXExprContainer {
                    span: DUMMY_SP,
                    expr: JSXExpr::Expr(expression),
                }));
            }
            return;
        }

        if key.starts_with("on") {
            self.transform_event_attribute(key, expression, elem_expr(), results);
            return;
        }

        let Some(expression) = expression else { return };
        let is_effect_candidate = !self.config.effect_wrapper.is_empty()
            && (self.is_dynamic(&expression, DynamicOpts::new().member())
                || ((key == "classList" || key == "style")
                    && evaluate(&expression, &self.bindings).is_none()));

        if is_effect_candidate {
            let mut next_elem = elem.cloned().expect("a dynamic attribute needs an id");
            if key == "value" || key == "checked" {
                let effect = self.register_import(&self.config.effect_wrapper.clone());
                let body = self.set_attr(
                    elem_expr(),
                    key,
                    expression,
                    SetAttrOpts {
                        is_svg,
                        is_ce,
                        tag_name: tag_name.to_string(),
                        ..Default::default()
                    },
                );
                results.post_exprs.push(expr_stmt(call(
                    Box::new(Expr::Ident(effect)),
                    vec![arg(arrow_expr(Vec::new(), body))],
                )));
                return;
            }
            if key == "textContent" {
                next_elem = self.uid.generate("el$");
                let mut text = JSXText {
                    span: DUMMY_SP,
                    value: " ".into(),
                    raw: " ".into(),
                };
                text.raw = " ".into();
                *children = Some(JSXElementChild::JSXText(text));
                results.declarations.push(VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(next_elem.clone().into()),
                    init: Some(member(elem_expr(), "firstChild")),
                    definite: false,
                });
            }
            results.dynamics.push(Dynamic {
                elem: next_elem,
                key: key.to_string(),
                value: expression,
                is_svg,
                is_ce,
                tag_name: tag_name.to_string(),
            });
            return;
        }

        let _ = node;
        let expr = self.set_attr(
            elem_expr(),
            key,
            expression,
            SetAttrOpts {
                is_svg,
                is_ce,
                tag_name: tag_name.to_string(),
                ..Default::default()
            },
        );
        results.exprs.push(expr_stmt(expr));
    }

    fn transform_event_attribute(
        &mut self,
        key: &str,
        expression: Option<Box<Expr>>,
        elem: Box<Expr>,
        results: &mut Results,
    ) {
        let Some(handler) = expression else { return };
        let event = to_event_name(key);

        if key.starts_with("on:") || key.starts_with("oncapture:") {
            let mut args = vec![
                arg(Box::new(str_lit(key.split(':').nth(1).unwrap()))),
                arg(handler),
            ];
            if key.starts_with("oncapture:") {
                args.push(bool_arg(true));
            }
            results
                .exprs
                .push(expr_stmt(call(member(elem, "addEventListener"), args)));
            return;
        }

        let delegated = self.config.delegate_events
            && (DELEGATED_EVENTS.contains(&&*event)
                || self.config.delegated_events.contains(&event));

        if delegated {
            // Only delegated events can be hydrated.
            results.has_hydratable_event = true;
            if !self.events.contains(&event) {
                self.events.push(event.clone());
            }
            let resolvable = self.detect_resolvable_event_handler(&handler);
            if let Expr::Array(array) = &*handler {
                let mut elements = array.elems.clone();
                if elements.len() > 1 {
                    let data = elements[1]
                        .take()
                        .expect("array holes are not handlers")
                        .expr;
                    results.exprs.insert(
                        0,
                        expr_stmt(Box::new(Expr::Assign(AssignExpr {
                            span: DUMMY_SP,
                            op: op!("="),
                            left: expr_to_assign_target(member(
                                elem.clone(),
                                &format!("$${event}Data"),
                            )),
                            right: data,
                        }))),
                    );
                }
                let first = elements
                    .first_mut()
                    .and_then(|el| el.take())
                    .expect("a handler array is never empty")
                    .expr;
                results.exprs.insert(
                    0,
                    expr_stmt(Box::new(Expr::Assign(AssignExpr {
                        span: DUMMY_SP,
                        op: op!("="),
                        left: expr_to_assign_target(member(elem, &format!("$${event}"))),
                        right: first,
                    }))),
                );
            } else if matches!(&*handler, Expr::Fn(_) | Expr::Arrow(_)) || resolvable {
                results.exprs.insert(
                    0,
                    expr_stmt(Box::new(Expr::Assign(AssignExpr {
                        span: DUMMY_SP,
                        op: op!("="),
                        left: expr_to_assign_target(member(elem, &format!("$${event}"))),
                        right: handler,
                    }))),
                );
            } else {
                let add = self.register_import("addEventListener");
                results.exprs.insert(
                    0,
                    expr_stmt(call(
                        Box::new(Expr::Ident(add)),
                        vec![
                            arg(elem),
                            arg(Box::new(str_lit(&event))),
                            arg(handler),
                            bool_arg(true),
                        ],
                    )),
                );
            }
            return;
        }

        let resolvable = self.detect_resolvable_event_handler(&handler);
        if let Expr::Array(array) = &*handler {
            let mut elements = array.elems.clone();
            let handler = if elements.len() > 1 {
                let first = elements[0]
                    .take()
                    .expect("array holes are not handlers")
                    .expr;
                let second = elements[1]
                    .take()
                    .expect("array holes are not handlers")
                    .expr;
                arrow_expr(
                    vec![Pat::Ident(ident("e").into())],
                    call(
                        first,
                        vec![arg(second), arg(Box::new(Expr::Ident(ident("e"))))],
                    ),
                )
            } else {
                elements[0]
                    .take()
                    .expect("array holes are not handlers")
                    .expr
            };
            results.exprs.insert(
                0,
                expr_stmt(call(
                    member(elem, "addEventListener"),
                    vec![arg(Box::new(str_lit(&event))), arg(handler)],
                )),
            );
        } else if matches!(&*handler, Expr::Fn(_) | Expr::Arrow(_)) || resolvable {
            results.exprs.insert(
                0,
                expr_stmt(call(
                    member(elem, "addEventListener"),
                    vec![arg(Box::new(str_lit(&event))), arg(handler)],
                )),
            );
        } else {
            let add = self.register_import("addEventListener");
            results.exprs.insert(
                0,
                expr_stmt(call(
                    Box::new(Expr::Ident(add)),
                    vec![arg(elem), arg(Box::new(str_lit(&event))), arg(handler)],
                )),
            );
        }
    }
}

fn typeof_is_function(id: &Ident) -> Box<Expr> {
    Box::new(Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: op!("==="),
        left: Box::new(Expr::Unary(UnaryExpr {
            span: DUMMY_SP,
            op: op!("typeof"),
            arg: Box::new(Expr::Ident(id.clone())),
        })),
        right: Box::new(str_lit("function")),
    }))
}

// Takes the box because it unwraps into boxes the AST already owns.
#[allow(clippy::boxed_local)]
pub(crate) fn strip_ts_wrappers(mut expr: Box<Expr>) -> Box<Expr> {
    loop {
        expr = match *expr {
            Expr::TsNonNull(inner) => inner.expr,
            Expr::TsAs(inner) => inner.expr,
            Expr::TsSatisfies(inner) => inner.expr,
            other => return Box::new(other),
        };
    }
}

fn is_lval(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Ident(_) | Expr::Member(_) | Expr::Array(_) | Expr::Object(_) | Expr::SuperProp(_)
    )
}

// ---------------------------------------------------------------- attribute preprocessing

fn attr_named(attr: &JSXAttrOrSpread, name: &str) -> bool {
    matches!(attr, JSXAttrOrSpread::JSXAttr(JSXAttr { name: JSXAttrName::Ident(id), .. }) if id.sym == *name)
}

fn object_value(attr: &JSXAttrOrSpread) -> Option<&ObjectLit> {
    let JSXAttrOrSpread::JSXAttr(JSXAttr {
        value: Some(JSXAttrValue::JSXExprContainer(container)),
        ..
    }) = attr
    else {
        return None;
    };
    let JSXExpr::Expr(expr) = &container.expr else {
        return None;
    };
    match &**expr {
        Expr::Object(object) => Some(object),
        _ => None,
    }
}

fn object_value_mut(attr: &mut JSXAttrOrSpread) -> Option<&mut ObjectLit> {
    let JSXAttrOrSpread::JSXAttr(JSXAttr {
        value: Some(JSXAttrValue::JSXExprContainer(container)),
        ..
    }) = attr
    else {
        return None;
    };
    let JSXExpr::Expr(expr) = &mut container.expr else {
        return None;
    };
    match &mut **expr {
        Expr::Object(object) => Some(object),
        _ => None,
    }
}

fn prop_key(prop: &KeyValueProp) -> Option<String> {
    match &prop.key {
        PropName::Ident(id) => Some(id.sym.to_string()),
        PropName::Str(s) => Some(s.value.to_atom_lossy().to_string()),
        PropName::Num(n) => Some(crate::eval::number_to_string(n.value)),
        _ => None,
    }
}

/// Babel's parser normalises `{ foo }` to a key/value property, so shorthand has to read the
/// same way here. `None` means the object holds something this transform cannot split up.
pub(crate) fn key_value_props(object: &ObjectLit) -> Option<Vec<KeyValueProp>> {
    object
        .props
        .iter()
        .map(|prop| match prop {
            PropOrSpread::Prop(prop) => match &**prop {
                Prop::KeyValue(kv) => Some(kv.clone()),
                Prop::Shorthand(id) => Some(KeyValueProp {
                    key: PropName::Ident(IdentName::new(id.sym.clone(), id.span)),
                    value: Box::new(Expr::Ident(id.clone())),
                }),
                _ => None,
            },
            PropOrSpread::Spread(_) => None,
        })
        .collect()
}

fn namespaced_attr(namespace: &str, name: &str, value: Box<Expr>) -> JSXAttrOrSpread {
    JSXAttrOrSpread::JSXAttr(JSXAttr {
        span: DUMMY_SP,
        name: JSXAttrName::JSXNamespacedName(JSXNamespacedName {
            span: DUMMY_SP,
            ns: IdentName::new(namespace.into(), DUMMY_SP),
            name: IdentName::new(name.into(), DUMMY_SP),
        }),
        value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
            span: DUMMY_SP,
            expr: JSXExpr::Expr(value),
        })),
    })
}

/// `style={{ color: x }}` becomes `style:color={x}` per non-computed property.
/// The index arithmetic is upstream's, kept literally rather than tidied, because a mixed
/// computed/static object relies on exactly how its `splice` lands.
impl<C: Comments> Transform<C> {
    fn preprocess_style(&self, attrs: &mut Vec<JSXAttrOrSpread>) {
        let Some(index) = attrs.iter().position(|attr| {
            attr_named(attr, "style")
                && object_value(attr).is_some_and(|object| key_value_props(object).is_some())
        }) else {
            return;
        };
        let object_lo = object_value(&attrs[index]).unwrap().span.lo;
        let props = key_value_props(object_value(&attrs[index]).unwrap()).unwrap();
        let mut i: i64 = 0;
        for (prop_index, prop) in props.iter().enumerate() {
            if matches!(prop.key, PropName::Computed(_)) {
                continue;
            }
            let Some(key) = prop_key(prop) else { continue };
            self.copy_leading(object_lo, prop.value.span().lo);
            i += 1;
            attrs.insert(
                index + i as usize,
                namespaced_attr("style", &key, prop.value.clone()),
            );
            let object = object_value_mut(&mut attrs[index]).unwrap();
            js_splice_remove(&mut object.props, prop_index as i64 - i - 1);
        }
        if object_value(&attrs[index]).is_some_and(|object| object.props.is_empty()) {
            attrs.remove(index);
        }
    }

    /// `classList={{ a: cond, b: true }}` becomes `class:a={cond}` / `class="b"`.
    fn preprocess_class_list(&self, attrs: &mut Vec<JSXAttrOrSpread>) {
        let Some(index) = attrs.iter().position(|attr| {
        attr_named(attr, "classList")
            && object_value(attr).is_some_and(|object| {
                key_value_props(object).is_some_and(|props| {
                    props.iter().all(|prop| {
                        !matches!(prop.key, PropName::Computed(_))
                            && !matches!(&prop.key, PropName::Str(s)
                                if s.value.to_atom_lossy().contains(' ') || s.value.to_atom_lossy().contains(':'))
                    })
                })
            })
    }) else {
        return;
    };
        let object_lo = object_value(&attrs[index]).unwrap().span.lo;
        let props = key_value_props(object_value(&attrs[index]).unwrap()).unwrap();
        // Upstream splices the *paths* array, not the object, so the object keeps its properties
        // and only this local count decides whether the attribute is dropped.
        let mut remaining: Vec<()> = props.iter().map(|_| ()).collect();
        let mut i: i64 = 0;
        for (prop_index, prop) in props.iter().enumerate() {
            let Some(key) = prop_key(prop) else { continue };
            self.copy_leading(object_lo, prop.value.span().lo);
            let evaluated = evaluate(&prop.value, &self.bindings);
            match &evaluated {
                None => {
                    i += 1;
                    attrs.insert(
                        index + i as usize,
                        namespaced_attr("class", &key, prop.value.clone()),
                    );
                }
                Some(value) if value.truthy() => {
                    i += 1;
                    attrs.insert(
                        index + i as usize,
                        JSXAttrOrSpread::JSXAttr(JSXAttr {
                            span: DUMMY_SP,
                            name: JSXAttrName::Ident(IdentName::new("class".into(), DUMMY_SP)),
                            value: Some(JSXAttrValue::Str(Str {
                                span: DUMMY_SP,
                                value: key.into(),
                                raw: None,
                            })),
                        }),
                    );
                }
                Some(_) => {}
            }
            js_splice_remove(&mut remaining, prop_index as i64 - i - 1);
        }
        if remaining.is_empty() {
            attrs.remove(index);
        }
    }
}

/// Several `class`/`className` attributes fold into one template literal.
fn combine_class_attributes(attrs: &mut Vec<JSXAttrOrSpread>) {
    let class_indices: Vec<usize> = attrs
        .iter()
        .enumerate()
        .filter(|(_, attr)| attr_named(attr, "class") || attr_named(attr, "className"))
        .map(|(index, _)| index)
        .collect();
    if class_indices.len() < 2 {
        return;
    }
    let mut values: Vec<Box<Expr>> = Vec::new();
    let mut quasis: Vec<String> = vec![String::new()];
    for (nth, &index) in class_indices.iter().enumerate() {
        let is_last = nth == class_indices.len() - 1;
        let JSXAttrOrSpread::JSXAttr(attr) = &attrs[index] else {
            unreachable!()
        };
        match &attr.value {
            Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(expr),
                ..
            })) => {
                values.push(Box::new(Expr::Bin(BinExpr {
                    span: DUMMY_SP,
                    op: op!("||"),
                    left: expr.clone(),
                    right: Box::new(str_lit("")),
                })));
                quasis.push(if is_last { String::new() } else { " ".into() });
            }
            value => {
                let text = match value {
                    Some(JSXAttrValue::Str(s)) => s.value.to_atom_lossy().to_string(),
                    _ => String::new(),
                };
                let prev = quasis.pop().unwrap_or_default();
                quasis.push(format!("{prev}{text}{}", if is_last { "" } else { " " }));
            }
        }
    }
    let first = class_indices[0];
    let new_value = if values.is_empty() {
        JSXAttrValue::Str(Str {
            span: DUMMY_SP,
            value: quasis[0].clone().into(),
            raw: None,
        })
    } else {
        let count = quasis.len();
        JSXAttrValue::JSXExprContainer(JSXExprContainer {
            span: DUMMY_SP,
            expr: JSXExpr::Expr(Box::new(Expr::Tpl(Tpl {
                span: DUMMY_SP,
                exprs: values,
                quasis: quasis
                    .into_iter()
                    .enumerate()
                    .map(|(index, raw)| TplElement {
                        span: DUMMY_SP,
                        tail: index == count - 1,
                        cooked: Some(raw.clone().into()),
                        raw: raw.into(),
                    })
                    .collect(),
            }))),
        })
    };
    if let JSXAttrOrSpread::JSXAttr(attr) = &mut attrs[first] {
        attr.value = Some(new_value);
    }
    for &index in class_indices.iter().skip(1).rev() {
        attrs.remove(index);
    }
}

// ---------------------------------------------------------------- children

impl<C: Comments> Transform<C> {
    fn transform_children(
        &mut self,
        children: Vec<JSXElementChild>,
        tag_name: &str,
        results: &mut Results,
    ) {
        let mut temp_path = results.id.as_ref().map(|id| id.sym.to_string());
        let mut next_placeholder: Option<Ident> = None;
        let mut i = 0usize;

        let filtered = filter_children(children);
        let filtered_refs: Vec<&JSXElementChild> = filtered.iter().collect();
        let last_element = self.find_last_element(&filtered);

        let mut child_nodes: Vec<Results> = Vec::new();
        for (index, child) in filtered.iter().enumerate() {
            if let JSXElementChild::JSXFragment(fragment) = child {
                self.errors.push(
                    fragment.span,
                    format!(
                        "Fragments can only be used top level in JSX. \
                         Not used under a <{tag_name}>."
                    ),
                );
                continue;
            }
            let skip_id = results.id.is_none() || !self.detect_expressions(&filtered_refs, index);
            let info = Info {
                to_be_closed: results.to_be_closed.clone(),
                last_element: Some(index) == last_element,
                skip_id,
                parent_is_native: true,
                ..Default::default()
            };
            let Some(transformed) = self.transform_node(child.clone(), info) else {
                continue;
            };
            match child_nodes.last_mut() {
                Some(previous) if transformed.text && previous.text => {
                    previous.template.push_str(&transformed.template);
                }
                _ => child_nodes.push(transformed),
            }
        }

        let multi = check_length(&filtered);
        let ids: Vec<Option<Ident>> = child_nodes.iter().map(|child| child.id.clone()).collect();
        let texts: Vec<bool> = child_nodes.iter().map(|child| child.text).collect();

        for (index, child) in child_nodes.into_iter().enumerate() {
            results.template.push_str(&child.template);
            if let Some(child_id) = child.id.clone() {
                if child.tag_name.as_deref() == Some("head") {
                    if self.config.hydratable {
                        let create_component = self.register_import("createComponent");
                        let no_hydration = self.register_import("NoHydration");
                        results.exprs.push(expr_stmt(call(
                            Box::new(Expr::Ident(create_component)),
                            vec![
                                arg(Box::new(Expr::Ident(no_hydration))),
                                arg(Box::new(Expr::Object(ObjectLit {
                                    span: DUMMY_SP,
                                    props: Vec::new(),
                                }))),
                            ],
                        )));
                    }
                    continue;
                }
                let walk = member(
                    Box::new(Expr::Ident(ident(
                        temp_path
                            .as_deref()
                            .expect("a walked child needs a parent id"),
                    ))),
                    if i == 0 { "firstChild" } else { "nextSibling" },
                );
                let init = if self.config.hydratable && tag_name == "html" {
                    let get_next_match = self.register_import("getNextMatch");
                    call(
                        Box::new(Expr::Ident(get_next_match)),
                        vec![
                            arg(walk),
                            arg(Box::new(str_lit(child.tag_name.as_deref().unwrap_or("")))),
                        ],
                    )
                } else {
                    walk
                };
                results.declarations.push(VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(child_id.clone().into()),
                    init: Some(init),
                    definite: false,
                });
                results.declarations.extend(child.declarations);
                results.exprs.extend(child.exprs);
                results.dynamics.extend(child.dynamics);
                results.post_exprs.extend(child.post_exprs);
                results.has_hydratable_event |= child.has_hydratable_event;
                results.has_custom_element |= child.has_custom_element;
                temp_path = Some(child_id.sym.to_string());
                next_placeholder = None;
                i += 1;
            } else if !child.exprs.is_empty() {
                let insert = self.register_import("insert");
                let parent = results
                    .id
                    .clone()
                    .expect("an inserted child needs a parent id");
                let value = child.into_first_expr();
                let markers = self.config.hydratable && multi;
                if markers || wrapped_by_text(&ids, &texts, index) {
                    if markers {
                        let id =
                            self.create_placeholder(results, temp_path.as_deref().unwrap(), i, "#");
                        i += 1;
                        temp_path = Some(id.0.sym.to_string());
                    }
                    let (expr_id, content_id) = match (&next_placeholder, markers) {
                        (Some(id), false) => (id.clone(), None),
                        _ => {
                            let created = self.create_placeholder(
                                results,
                                temp_path.as_deref().unwrap(),
                                i,
                                if markers { "/" } else { "" },
                            );
                            i += 1;
                            created
                        }
                    };
                    if !markers {
                        next_placeholder = Some(expr_id.clone());
                    }
                    let mut args = vec![
                        arg(Box::new(Expr::Ident(parent))),
                        arg(value),
                        arg(Box::new(Expr::Ident(expr_id.clone()))),
                    ];
                    args.extend(content_id.map(|id| arg(Box::new(Expr::Ident(id)))));
                    results
                        .exprs
                        .push(expr_stmt(call(Box::new(Expr::Ident(insert)), args)));
                    temp_path = Some(expr_id.sym.to_string());
                } else if multi {
                    let next = next_child(&ids, index)
                        .map(|id| Box::new(Expr::Ident(id)))
                        .unwrap_or_else(|| Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))));
                    results.exprs.push(expr_stmt(call(
                        Box::new(Expr::Ident(insert)),
                        vec![arg(Box::new(Expr::Ident(parent))), arg(value), arg(next)],
                    )));
                } else {
                    results.exprs.push(expr_stmt(call(
                        Box::new(Expr::Ident(insert)),
                        vec![arg(Box::new(Expr::Ident(parent))), arg(value)],
                    )));
                }
            } else {
                next_placeholder = None;
            }
        }
    }

    /// Returns the placeholder and, when hydrating a closing marker, the content id that
    /// `getNextMarker` hands back alongside it.
    fn create_placeholder(
        &mut self,
        results: &mut Results,
        temp_path: &str,
        i: usize,
        char: &str,
    ) -> (Ident, Option<Ident>) {
        let expr_id = self.uid.generate("el$");
        results.template.push_str(&format!("<!{char}>"));
        if self.config.hydratable && char == "/" {
            let content_id = self.uid.generate("co$");
            let get_next_marker = self.register_import("getNextMarker");
            results.declarations.push(VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Array(ArrayPat {
                    span: DUMMY_SP,
                    elems: vec![
                        Some(Pat::Ident(expr_id.clone().into())),
                        Some(Pat::Ident(content_id.clone().into())),
                    ],
                    optional: false,
                    type_ann: None,
                }),
                init: Some(call(
                    Box::new(Expr::Ident(get_next_marker)),
                    vec![arg(member(
                        Box::new(Expr::Ident(ident(temp_path))),
                        "nextSibling",
                    ))],
                )),
                definite: false,
            });
            return (expr_id, Some(content_id));
        }
        results.declarations.push(VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(expr_id.clone().into()),
            init: Some(member(
                Box::new(Expr::Ident(ident(temp_path))),
                if i == 0 { "firstChild" } else { "nextSibling" },
            )),
            definite: false,
        });
        (expr_id, None)
    }

    fn find_last_element(&self, children: &[JSXElementChild]) -> Option<usize> {
        for (index, child) in children.iter().enumerate().rev() {
            let hit = self.config.hydratable
                || matches!(child, JSXElementChild::JSXText(_))
                || self.get_static_expression(child, false).is_some()
                || matches!(child, JSXElementChild::JSXElement(el)
                    if !is_component(&get_tag_name(el)));
            if hit {
                return Some(index);
            }
        }
        None
    }

    /// `detectExpressions`: whether this element still needs a reference at runtime.
    ///
    /// Called once per child, and it walks the whole remaining subtree each time, so it must
    /// not copy anything: it reads through references throughout.
    fn detect_expressions(&self, children: &[&JSXElementChild], index: usize) -> bool {
        if index > 0 {
            let previous = children[index - 1];
            if matches!(previous, JSXElementChild::JSXExprContainer(_))
                && !is_empty_expr_container(previous)
                && self.get_static_expression(previous, false).is_none()
            {
                return true;
            }
            if let JSXElementChild::JSXElement(el) = previous
                && is_component(&get_tag_name(el))
            {
                return true;
            }
        }
        for child in children.iter().skip(index) {
            match *child {
                JSXElementChild::JSXExprContainer(_) => {
                    if !is_empty_expr_container(child)
                        && self.get_static_expression(child, false).is_none()
                    {
                        return true;
                    }
                }
                JSXElementChild::JSXElement(el) => {
                    let tag_name = get_tag_name(el);
                    if is_component(&tag_name) {
                        return true;
                    }
                    if self.config.context_to_custom_elements
                        && (tag_name == "slot" || tag_name.contains('-'))
                    {
                        return true;
                    }
                    let interesting = el.opening.attrs.iter().any(|attr| match attr {
                        JSXAttrOrSpread::SpreadElement(_) => true,
                        JSXAttrOrSpread::JSXAttr(attr) => {
                            let name = match &attr.name {
                                JSXAttrName::Ident(id) => id.sym.to_string(),
                                JSXAttrName::JSXNamespacedName(ns) => {
                                    if ns.ns.sym == *"use" {
                                        return true;
                                    }
                                    ns.name.sym.to_string()
                                }
                            };
                            if ["textContent", "innerHTML", "innerText"].contains(&&*name) {
                                return true;
                            }
                            matches!(&attr.value, Some(JSXAttrValue::JSXExprContainer(container))
                                if !matches!(&container.expr, JSXExpr::Expr(expr)
                                    if matches!(&**expr, Expr::Lit(Lit::Str(_)) | Expr::Lit(Lit::Num(_)))))
                        }
                    });
                    if interesting {
                        return true;
                    }
                    let next = filter_children_ref(&el.children);
                    if !next.is_empty() && self.detect_expressions(&next, 0) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// `processSpreads`.
    fn process_spreads(
        &mut self,
        attributes: Vec<JSXAttrOrSpread>,
        elem: Box<Expr>,
        is_svg: bool,
        has_children: bool,
    ) -> (Vec<JSXAttrOrSpread>, Stmt) {
        let mut filtered = Vec::new();
        let mut spread_args: Vec<Box<Expr>> = Vec::new();
        let mut running_object: Vec<PropOrSpread> = Vec::new();
        let mut dynamic_spread = false;
        let mut first_spread = false;

        for attribute in attributes {
            match attribute {
                JSXAttrOrSpread::SpreadElement(spread) => {
                    first_spread = true;
                    if !running_object.is_empty() {
                        spread_args.push(Box::new(Expr::Object(ObjectLit {
                            span: DUMMY_SP,
                            props: std::mem::take(&mut running_object),
                        })));
                    }
                    let argument = spread.expr;
                    if self.is_dynamic(&argument, DynamicOpts::new().member()) {
                        dynamic_spread = true;
                        spread_args.push(unwrap_zero_arg_call(argument));
                    } else {
                        spread_args.push(argument);
                    }
                }
                JSXAttrOrSpread::JSXAttr(attr) => {
                    let key = attr_key(&attr.name);
                    let container_expr = match &attr.value {
                        Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            expr: JSXExpr::Expr(expr),
                            ..
                        })) => Some(expr.clone()),
                        _ => None,
                    };
                    let dynamic = container_expr
                        .as_ref()
                        .is_some_and(|expr| self.is_dynamic(expr, DynamicOpts::new().member()));
                    if (first_spread || dynamic) && can_native_spread(&key, true) {
                        if dynamic {
                            let expr = container_expr.unwrap();
                            let id = convert_jsx_identifier(&attr.name);
                            let body = if self.config.wrap_conditionals
                                && (matches!(&*expr, Expr::Cond(_))
                                    || matches!(&*expr, Expr::Bin(bin) if is_logical(bin.op)))
                            {
                                match self.transform_condition(expr, true, false) {
                                    Condition::Expr(arrow) => arrow_body(arrow),
                                    Condition::Hoisted(..) => unreachable!("inline never hoists"),
                                }
                            } else {
                                expr
                            };
                            running_object.push(getter(id, !is_valid_identifier(&key), body));
                        } else {
                            let value = match container_expr {
                                Some(expr) => expr,
                                None => match &attr.value {
                                    Some(JSXAttrValue::Str(s)) => {
                                        Box::new(Expr::Lit(Lit::Str(s.clone())))
                                    }
                                    _ if PROPERTIES.contains(&&*key) => {
                                        Box::new(Expr::Lit(Lit::Bool(Bool {
                                            span: DUMMY_SP,
                                            value: true,
                                        })))
                                    }
                                    _ => Box::new(str_lit("")),
                                },
                            };
                            running_object.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                KeyValueProp {
                                    key: PropName::Str(Str {
                                        span: DUMMY_SP,
                                        value: key.clone().into(),
                                        raw: None,
                                    }),
                                    value,
                                },
                            ))));
                        }
                    } else {
                        filtered.push(JSXAttrOrSpread::JSXAttr(attr));
                    }
                }
            }
        }

        if !running_object.is_empty() {
            spread_args.push(Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: running_object,
            })));
        }

        let props = if spread_args.len() == 1 && !dynamic_spread {
            spread_args.remove(0)
        } else {
            let merge = self.register_import("mergeProps");
            call(
                Box::new(Expr::Ident(merge)),
                spread_args.into_iter().map(arg).collect(),
            )
        };
        let spread = self.register_import("spread");
        (
            filtered,
            expr_stmt(call(
                Box::new(Expr::Ident(spread)),
                vec![
                    arg(elem),
                    arg(props),
                    bool_arg(is_svg),
                    bool_arg(has_children),
                ],
            )),
        )
    }
}

/// `isCallExpression(arg) && !arg.arguments.length && !isCall(callee) && !isMember(callee)`
/// unwraps to the callee; anything else is wrapped in a thunk.
pub(crate) fn unwrap_zero_arg_call(expr: Box<Expr>) -> Box<Expr> {
    if let Expr::Call(call) = &*expr
        && call.args.is_empty()
        && let Callee::Expr(callee) = &call.callee
        && !matches!(&**callee, Expr::Call(_) | Expr::Member(_))
    {
        return callee.clone();
    }
    arrow_expr(Vec::new(), expr)
}

// Takes the box because it unwraps into boxes the AST already owns.
#[allow(clippy::boxed_local)]
pub(crate) fn arrow_body(arrow: Box<Expr>) -> Box<Expr> {
    match *arrow {
        Expr::Arrow(arrow) => match *arrow.body {
            ArrowFunctionBody::Expr(expr) => expr,
            ArrowFunctionBody::FunctionBody(block) => Box::new(Expr::Arrow(ArrowExpr {
                span: DUMMY_SP,
                params: Vec::new(),
                body: Box::new(ArrowFunctionBody::FunctionBody(block)),
                is_async: false,
                is_generator: false,
                ..Default::default()
            })),
        },
        other => Box::new(other),
    }
}

pub(crate) fn getter(key: PropName, computed: bool, body: Box<Expr>) -> PropOrSpread {
    let key = if !computed {
        key
    } else {
        PropName::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: match key {
                PropName::Ident(id) => Box::new(str_lit(&id.sym)),
                PropName::Str(s) => Box::new(Expr::Lit(Lit::Str(s))),
                other => Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: format!("{other:?}").into(),
                    raw: None,
                }))),
            },
        })
    };
    PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
        span: DUMMY_SP,
        key,
        function: crate::template::getter_fn(
            Vec::new(),
            crate::template::fn_body(vec![Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(body),
            })]),
        ),
    })))
}

fn next_child(ids: &[Option<Ident>], index: usize) -> Option<Ident> {
    ids.get(index + 1)
        .and_then(|id| id.clone().or_else(|| next_child(ids, index + 1)))
}
