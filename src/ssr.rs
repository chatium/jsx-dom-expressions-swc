//! The `generate: "ssr"` renderer: a list of static HTML chunks with expressions interleaved,
//! rather than a single template string walked at runtime.

use swc_core::common::comments::Comments;
use swc_core::common::{DUMMY_SP, Spanned};
use swc_core::ecma::ast::*;

use crate::Transform;
use crate::constants::{
    ALIASES, CHILD_PROPERTIES, PROPERTIES, RESERVED_NAMESPACES, SVG_ELEMENTS, VOID_ELEMENTS,
};
use crate::element::{js_splice_remove, key_value_props, unwrap_zero_arg_call};
use crate::template::{arg, arrow_expr, bool_arg, call, expr_stmt, fn_body, getter_fn, str_lit};
use crate::transform::{Info, Results, decode_html_entities};
use crate::utils::{
    DynamicOpts, attr_key, check_length, convert_jsx_identifier, escape_html, filter_children,
    get_tag_name, ident, is_component, is_valid_identifier, prop_key_name, trim_whitespace,
};

/// `BooleanAttributes` is `PROPERTIES` minus the camel-cased aliases that lead it.
fn is_boolean_attribute(key: &str) -> bool {
    PROPERTIES.iter().skip(7).any(|attribute| *attribute == key)
}

/// `appendToTemplate`: the head of `value` extends the open chunk, the rest become new chunks.
fn append_to_template(template: &mut Vec<String>, value: &[String]) {
    let Some((first, rest)) = value.split_first() else {
        return;
    };
    template
        .last_mut()
        .expect("a template always has an open chunk")
        .push_str(first);
    template.extend_from_slice(rest);
}

fn to_attribute(key: &str, is_svg: bool) -> String {
    let key = ALIASES
        .iter()
        .find(|(from, _)| *from == key)
        .map(|(_, to)| *to)
        .unwrap_or(key);
    if is_svg {
        key.to_string()
    } else {
        key.to_lowercase()
    }
}

impl<C: Comments> Transform<C> {
    pub(crate) fn transform_element_ssr(&mut self, mut el: JSXElement, info: Info) -> Results {
        if el
            .opening
            .attrs
            .iter()
            .any(|attr| matches!(attr, JSXAttrOrSpread::SpreadElement(_)))
        {
            return self.create_element_ssr(el, &info);
        }

        let tag_name = get_tag_name(&el);
        let void_tag = VOID_ELEMENTS.contains(&&*tag_name);
        let mut results = Results {
            template_parts: vec![format!("<{tag_name}")],
            tag_name: Some(tag_name.clone()),
            wont_escape: self.wont_escape.contains(&el.span().lo),
            ..Default::default()
        };
        let mut do_not_escape = info.do_not_escape || tag_name == "script" || tag_name == "style";

        if info.top_level && self.config.hydratable {
            if tag_name == "head" {
                // Registration order is observable, and upstream registers both before it
                // recurses into the element.
                let no_hydration = self.register_import("NoHydration");
                let create_component = self.register_import("createComponent");
                let child = self.transform_element_ssr(
                    el,
                    Info {
                        top_level: false,
                        ..info.clone()
                    },
                );
                results.template_parts = Vec::new();
                let wrapped = self.wrap_no_hydration(no_hydration, create_component, child);
                results.exprs.push(expr_stmt(wrapped));
                return results;
            }
            results.template_parts.push(String::new());
            let key = self.register_import("ssrHydrationKey");
            results
                .template_values
                .push(call(Box::new(Expr::Ident(key)), Vec::new()));
        }

        self.transform_attributes_ssr(&mut el, &mut results, &mut do_not_escape);
        append_to_template(&mut results.template_parts, &[">".into()]);
        if !void_tag {
            let children = std::mem::take(&mut el.children);
            self.transform_children_ssr(children, &mut results, do_not_escape);
            append_to_template(&mut results.template_parts, &[format!("</{tag_name}>")]);
        }
        results
    }

    fn no_hydration_component(&mut self, child: Results) -> Box<Expr> {
        // Registration order is observable: imports print in reverse, and SSR registers
        // NoHydration before createComponent.
        let no_hydration = self.register_import("NoHydration");
        let create_component = self.register_import("createComponent");
        self.wrap_no_hydration(no_hydration, create_component, child)
    }

    fn wrap_no_hydration(
        &mut self,
        no_hydration: Ident,
        create_component: Ident,
        child: Results,
    ) -> Box<Expr> {
        let body = self.create_template_ssr(child);
        call(
            Box::new(Expr::Ident(create_component)),
            vec![
                arg(Box::new(Expr::Ident(no_hydration))),
                arg(Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: vec![PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
                        span: DUMMY_SP,
                        key: PropName::Ident(IdentName::new("children".into(), DUMMY_SP)),
                        function: getter_fn(
                            Vec::new(),
                            fn_body(vec![Stmt::Return(ReturnStmt {
                                span: DUMMY_SP,
                                arg: Some(body),
                            })]),
                        ),
                    })))],
                }))),
            ],
        )
    }

    /// `createTemplate` for SSR: register the chunk list and call `ssr(...)` with the values.
    pub(crate) fn create_template_ssr(&mut self, mut result: Results) -> Box<Expr> {
        if result.template_parts.is_empty() {
            return result.into_first_expr();
        }
        let chunks = result.template_parts.clone();
        let id = match self
            .ssr_templates
            .iter()
            .find(|(_, existing)| *existing == chunks)
        {
            Some((id, _)) => id.clone(),
            None => {
                let id = self.uid.generate("tmpl$");
                self.ssr_templates.push((id.clone(), chunks.clone()));
                id
            }
        };

        if result.wont_escape {
            if chunks.len() == 1 {
                return Box::new(Expr::Ident(id));
            }
            // A chunk pair whose only value is the hydration key needs no `ssr()` call.
            if chunks.len() == 2
                && result
                    .template_values
                    .first()
                    .is_some_and(|value| self.is_hydration_key_call(value))
            {
                let index = |n: f64| {
                    Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::Ident(id.clone())),
                        prop: MemberProp::Computed(ComputedPropName {
                            span: DUMMY_SP,
                            expr: Box::new(Expr::Lit(Lit::Num(Number {
                                span: DUMMY_SP,
                                value: n,
                                raw: None,
                            }))),
                        }),
                    }))
                };
                return Box::new(Expr::Bin(BinExpr {
                    span: DUMMY_SP,
                    op: op!(bin, "+"),
                    left: Box::new(Expr::Bin(BinExpr {
                        span: DUMMY_SP,
                        op: op!(bin, "+"),
                        left: index(0.0),
                        right: result.template_values.remove(0),
                    })),
                    right: index(1.0),
                }));
            }
        }

        let ssr = self.register_import("ssr");
        let mut args = vec![arg(Box::new(Expr::Ident(id)))];
        if chunks.len() > 1 {
            args.extend(result.template_values.into_iter().map(arg));
        }
        call(Box::new(Expr::Ident(ssr)), args)
    }

    fn is_hydration_key_call(&self, expr: &Expr) -> bool {
        let Expr::Call(call) = expr else { return false };
        let Callee::Expr(callee) = &call.callee else {
            return false;
        };
        let Expr::Ident(name) = &**callee else {
            return false;
        };
        self.imports
            .iter()
            .any(|(key, id)| key == "ssrHydrationKey" && id.sym == name.sym)
    }

    /// `appendTemplates` for SSR: a plain string, or an array of chunks.
    pub(crate) fn append_templates_ssr(&mut self) -> Option<Stmt> {
        if self.ssr_templates.is_empty() {
            return None;
        }
        let decls = std::mem::take(&mut self.ssr_templates)
            .into_iter()
            .map(|(id, chunks)| VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(id.into()),
                init: Some(if chunks.len() == 1 {
                    Box::new(str_lit(&chunks[0]))
                } else {
                    Box::new(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: chunks
                            .iter()
                            .map(|chunk| Some(arg(Box::new(str_lit(chunk)))))
                            .collect(),
                    }))
                }),
                definite: false,
            })
            .collect();
        Some(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Const,
            declare: false,
            decls,
            ..Default::default()
        }))))
    }
}

// ---------------------------------------------------------------- attributes

impl<C: Comments> Transform<C> {
    /// `normalizeAttributes`: fold `style:*` / `class:*` namespaces into objects and merge the
    /// class family into one attribute. Upstream works on a local copy of the attribute list,
    /// leaving the JSX node untouched, so this returns the list rather than mutating the node.
    fn normalize_attributes(&mut self, el: &JSXElement) -> Vec<JSXAttrOrSpread> {
        let mut attributes = el.opening.attrs.clone();
        let namespaced = |attrs: &[JSXAttrOrSpread], namespace: &str| -> Vec<usize> {
            attrs
                .iter()
                .enumerate()
                .filter(|(_, attr)| {
                    matches!(attr, JSXAttrOrSpread::JSXAttr(JSXAttr {
                        name: JSXAttrName::JSXNamespacedName(ns), ..
                    }) if ns.ns.sym == *namespace)
                })
                .map(|(index, _)| index)
                .collect()
        };

        let class_namespaced = namespaced(&attributes, "class");
        if !class_namespaced.is_empty() {
            transform_to_object("classList", &mut attributes, &class_namespaced);
        }

        let class_indices: Vec<usize> = attributes
            .iter()
            .enumerate()
            .filter(|(_, attr)| {
                matches!(attr, JSXAttrOrSpread::JSXAttr(JSXAttr {
                    name: JSXAttrName::Ident(id), ..
                }) if matches!(&*id.sym, "class" | "className" | "classList"))
            })
            .map(|(index, _)| index)
            .collect();
        if class_indices.len() > 1 {
            self.combine_class_attributes_ssr(&mut attributes, &class_indices);
        }

        let style_namespaced = namespaced(&attributes, "style");
        if !style_namespaced.is_empty() {
            transform_to_object("style", &mut attributes, &style_namespaced);
        }
        attributes
    }

    fn combine_class_attributes_ssr(
        &mut self,
        attributes: &mut Vec<JSXAttrOrSpread>,
        class_indices: &[usize],
    ) {
        let mut values: Vec<Box<Expr>> = Vec::new();
        let mut quasis: Vec<String> = vec![String::new()];
        let mut drop = Vec::new();
        for (nth, &index) in class_indices.iter().enumerate() {
            let is_last = nth == class_indices.len() - 1;
            let JSXAttrOrSpread::JSXAttr(attr) = attributes[index].clone() else {
                continue;
            };
            let name = attr_key(&attr.name);
            match attr.value {
                Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                    expr: JSXExpr::Expr(expr),
                    ..
                })) => {
                    let mut expr = expr;
                    if name == "classList" {
                        if let Expr::Object(object) = &*expr
                            && !object
                                .props
                                .iter()
                                .any(|prop| matches!(prop, PropOrSpread::Spread(_)))
                        {
                            self.transform_classlist_object(
                                object.clone(),
                                &mut values,
                                &mut quasis,
                            );
                            if nth > 0 {
                                drop.push(index);
                            }
                            continue;
                        }
                        let class_list = self.register_import("ssrClassList");
                        expr = call(Box::new(Expr::Ident(class_list)), vec![arg(expr)]);
                    }
                    values.push(Box::new(Expr::Bin(BinExpr {
                        span: DUMMY_SP,
                        op: op!("||"),
                        left: expr,
                        right: Box::new(str_lit("")),
                    })));
                    quasis.push(if is_last { String::new() } else { " ".into() });
                }
                value => {
                    let text = match &value {
                        Some(JSXAttrValue::Str(s)) => s.value.to_atom_lossy().to_string(),
                        _ => String::new(),
                    };
                    let prev = quasis.pop().unwrap_or_default();
                    quasis.push(format!(
                        "{prev}{}{text}{}",
                        if nth > 0 { " " } else { "" },
                        if is_last { "" } else { " " }
                    ));
                }
            }
            if nth > 0 {
                drop.push(index);
            }
        }
        let count = quasis.len();
        if let JSXAttrOrSpread::JSXAttr(attr) = &mut attributes[class_indices[0]] {
            attr.name = JSXAttrName::Ident(IdentName::new("class".into(), DUMMY_SP));
            attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
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
            }));
        }
        drop.sort_unstable();
        for index in drop.into_iter().rev() {
            attributes.remove(index);
        }
    }

    #[allow(clippy::vec_box)] // these go straight into the AST, which wants `Box<Expr>`.
    fn transform_classlist_object(
        &mut self,
        object: ObjectLit,
        values: &mut Vec<Box<Expr>>,
        quasis: &mut Vec<String>,
    ) {
        let props = key_value_props(&object).unwrap_or_default();
        let count = props.len();
        for (index, prop) in props.into_iter().enumerate() {
            let is_last = index + 1 == count;
            let (key, computed) = match &prop.key {
                PropName::Ident(id) => (Box::new(str_lit(&id.sym)), false),
                PropName::Computed(computed) => {
                    let escape = self.register_import("escape");
                    (
                        call(
                            Box::new(Expr::Ident(escape)),
                            vec![arg(computed.expr.clone()), bool_arg(true)],
                        ),
                        true,
                    )
                }
                PropName::Str(s) => (
                    Box::new(str_lit(&escape_html(&s.value.to_atom_lossy(), false))),
                    false,
                ),
                PropName::Num(n) => (
                    Box::new(str_lit(&escape_html(
                        &crate::eval::number_to_string(n.value),
                        false,
                    ))),
                    false,
                ),
                PropName::BigInt(_) => (Box::new(str_lit("")), false),
            };
            if let Expr::Lit(Lit::Bool(flag)) = &*prop.value {
                if !flag.value {
                    continue;
                }
                if computed {
                    values.push(key);
                    quasis.push(if is_last { String::new() } else { " ".into() });
                } else {
                    let literal = match &*key {
                        Expr::Lit(Lit::Str(s)) => s.value.to_atom_lossy().to_string(),
                        _ => String::new(),
                    };
                    let prev = quasis.pop().unwrap_or_default();
                    quasis.push(format!(
                        "{prev}{}{literal}{}",
                        if index > 0 { " " } else { "" },
                        if is_last { "" } else { " " }
                    ));
                }
                continue;
            }
            values.push(Box::new(Expr::Cond(CondExpr {
                span: DUMMY_SP,
                test: prop.value,
                cons: key,
                alt: Box::new(str_lit("")),
            })));
            quasis.push(if is_last { String::new() } else { " ".into() });
        }
    }
}

/// Upstream reads `t.isIdentifier(p.key) ? p.key.name : p.key.value`, which ignores whether
/// the key is computed: `{[cssVar]: v}` contributes the identifier's *name*, and a key that is
/// neither an identifier nor a literal has no `.value`, so JS stringifies `undefined`.
fn ssr_style_key(key: &PropName) -> String {
    let from_expr = |expr: &Expr| match expr {
        Expr::Ident(id) => id.sym.to_string(),
        Expr::Lit(Lit::Str(s)) => s.value.to_atom_lossy().to_string(),
        Expr::Lit(Lit::Num(n)) => crate::eval::number_to_string(n.value),
        _ => "undefined".to_string(),
    };
    match key {
        PropName::Ident(id) => id.sym.to_string(),
        PropName::Str(s) => s.value.to_atom_lossy().to_string(),
        PropName::Num(n) => crate::eval::number_to_string(n.value),
        PropName::BigInt(n) => n.value.to_string(),
        PropName::Computed(computed) => from_expr(&computed.expr),
    }
}

/// `transformToObject`: gather `style:x` / `class:x` attributes into one object-valued
/// attribute, merging into an existing `style` / `classList` object when there is one.
fn transform_to_object(attr_name: &str, attributes: &mut Vec<JSXAttrOrSpread>, selected: &[usize]) {
    let existing = attributes.iter().position(|attr| {
        matches!(attr, JSXAttrOrSpread::JSXAttr(JSXAttr {
            name: JSXAttrName::Ident(id), ..
        }) if id.sym == *attr_name)
    });
    let mut properties = Vec::new();
    let mut removals = Vec::new();
    for (i, &index) in selected.iter().enumerate() {
        let JSXAttrOrSpread::JSXAttr(attr) = attributes[index].clone() else {
            continue;
        };
        let JSXAttrName::JSXNamespacedName(ns) = &attr.name else {
            continue;
        };
        let name = ns.name.sym.to_string();
        let key = if is_valid_identifier(&name) {
            PropName::Ident(IdentName::new(name.clone().into(), DUMMY_SP))
        } else {
            PropName::Str(Str {
                span: DUMMY_SP,
                value: name.clone().into(),
                raw: None,
            })
        };
        let value = match attr.value {
            Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(expr),
                ..
            })) => expr,
            Some(JSXAttrValue::Str(s)) => Box::new(Expr::Lit(Lit::Str(s))),
            _ => Box::new(Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: true,
            }))),
        };
        properties.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key,
            value,
        }))));
        if existing.is_some() || i > 0 {
            removals.push(index);
        }
    }

    let merged = existing.and_then(|index| match &mut attributes[index] {
        JSXAttrOrSpread::JSXAttr(JSXAttr {
            value: Some(JSXAttrValue::JSXExprContainer(container)),
            ..
        }) => match &mut container.expr {
            JSXExpr::Expr(expr) => match &mut **expr {
                Expr::Object(object) => {
                    object.props.append(&mut properties);
                    Some(())
                }
                _ => None,
            },
            _ => None,
        },
        _ => None,
    });
    if merged.is_none() {
        attributes[selected[0]] = JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: DUMMY_SP,
            name: JSXAttrName::Ident(IdentName::new(attr_name.into(), DUMMY_SP)),
            value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                span: DUMMY_SP,
                expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: properties,
                }))),
            })),
        });
        removals.retain(|index| *index != selected[0]);
    }
    for index in removals {
        js_splice_remove(attributes, index as i64);
    }
}

impl<C: Comments> Transform<C> {
    fn transform_attributes_ssr(
        &mut self,
        el: &mut JSXElement,
        results: &mut Results,
        do_not_escape: &mut bool,
    ) {
        let tag_name = get_tag_name(el);
        let is_svg = SVG_ELEMENTS.contains(&&*tag_name);
        let has_children = !el.children.is_empty();
        let attributes = self.normalize_attributes(el);
        let mut children = None;

        for attribute in attributes {
            let JSXAttrOrSpread::JSXAttr(node) = attribute else {
                continue;
            };
            let mut key = attr_key(&node.name);
            let reserved_namespace = matches!(&node.name, JSXAttrName::JSXNamespacedName(ns)
                if RESERVED_NAMESPACES.contains(&&*ns.ns.sym));
            let mut value = node.value.clone();

            if (reserved_namespace || CHILD_PROPERTIES.contains(&&*key))
                && !matches!(value, Some(JSXAttrValue::JSXExprContainer(_)))
            {
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

            let container = match &value {
                Some(JSXAttrValue::JSXExprContainer(container)) => Some(container.clone()),
                _ => None,
            };
            let takes_expression_path = container.as_ref().is_some_and(|container| {
                reserved_namespace
                    || CHILD_PROPERTIES.contains(&&*key)
                    || !matches!(&container.expr, JSXExpr::Expr(inner)
                        if matches!(&**inner, Expr::Lit(Lit::Str(_)) | Expr::Lit(Lit::Num(_))))
            });

            if takes_expression_path {
                if key == "ref"
                    || key.starts_with("use:")
                    || key.starts_with("prop:")
                    || key.starts_with("on")
                {
                    continue;
                }
                let container = container.expect("checked above");
                let mut expression = match container.expr {
                    JSXExpr::Expr(expr) => expr,
                    JSXExpr::JSXEmptyExpr(_) => continue,
                };
                if CHILD_PROPERTIES.contains(&&*key) {
                    if self.config.hydratable && key == "textContent" {
                        expression = Box::new(Expr::Bin(BinExpr {
                            span: DUMMY_SP,
                            op: op!("||"),
                            left: expression,
                            right: Box::new(str_lit(" ")),
                        }));
                    }
                    if key == "innerHTML" {
                        *do_not_escape = true;
                    }
                    children = Some(JSXElementChild::JSXExprContainer(JSXExprContainer {
                        span: DUMMY_SP,
                        expr: JSXExpr::Expr(expression),
                    }));
                    continue;
                }

                let mut do_escape = true;
                if is_boolean_attribute(&key) {
                    let ssr_attribute = self.register_import("ssrAttribute");
                    results.template_parts.push(String::new());
                    results.template_values.push(call(
                        Box::new(Expr::Ident(ssr_attribute)),
                        vec![
                            arg(Box::new(str_lit(&key))),
                            arg(expression),
                            bool_arg(true),
                        ],
                    ));
                    continue;
                }
                if key == "style" {
                    expression = self.ssr_style(expression);
                    do_escape = false;
                }
                if key == "classList" {
                    expression = self.ssr_class_list(expression);
                    key = "class".into();
                    do_escape = false;
                }
                if do_escape {
                    expression = self.escape_expression(expression, true, false);
                }

                // Babel's `isLiteral` covers template literals too.
                // Babel's `isLiteral` covers template literals too, and its
                // `isBinaryExpression` excludes the logical operators SWC folds into `BinExpr`.
                if !do_escape
                    || matches!(&*expression, Expr::Lit(_) | Expr::Tpl(_))
                    || crate::transform::is_binary(&expression)
                {
                    let attribute_name = to_attribute(&key, is_svg);
                    append_to_template(
                        &mut results.template_parts,
                        &[format!(" {attribute_name}=\"")],
                    );
                    results.template_parts.push("\"".into());
                    results.template_values.push(expression);
                } else {
                    self.set_attr_ssr(results, &key, expression, is_svg);
                }
            } else {
                if key == "$ServerOnly" {
                    continue;
                }
                let value = match value {
                    Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        expr: JSXExpr::Expr(expr),
                        ..
                    })) => Some(*expr),
                    Some(JSXAttrValue::Str(s)) => Some(Expr::Lit(Lit::Str(s))),
                    _ => None,
                };
                key = to_attribute(&key, is_svg);
                append_to_template(&mut results.template_parts, &[format!(" {key}")]);
                // Babel reads `value.value`, which is a number for `width={20}`. The DOM path
                // never gets here with one because it folds the container to a string first.
                let mut text = match value {
                    Some(Expr::Lit(Lit::Str(text))) => text.value.to_atom_lossy().to_string(),
                    Some(Expr::Lit(Lit::Num(number))) => {
                        crate::eval::number_to_string(number.value)
                    }
                    _ => continue,
                };
                if key == "style" || key == "class" {
                    text = trim_whitespace(&text);
                    if key == "style" {
                        text = text.replace("; ", ";").replace(": ", ":");
                    }
                }
                append_to_template(
                    &mut results.template_parts,
                    &[format!("=\"{}\"", escape_html(&text, true))],
                );
            }
        }

        if !has_children && let Some(child) = children {
            el.children.push(child);
        }
    }

    fn ssr_style(&mut self, expression: Box<Expr>) -> Box<Expr> {
        if let Expr::Object(object) = &*expression
            && !object
                .props
                .iter()
                .any(|prop| matches!(prop, PropOrSpread::Spread(_)))
        {
            let mut parts: Vec<Box<Expr>> = Vec::new();
            for (index, prop) in key_value_props(object)
                .unwrap_or_default()
                .into_iter()
                .enumerate()
            {
                let name = ssr_style_key(&prop.key);
                let escaped = self.escape_expression(prop.value, true, true);
                parts.push(Box::new(Expr::Bin(BinExpr {
                    span: DUMMY_SP,
                    op: op!(bin, "+"),
                    left: Box::new(str_lit(&format!(
                        "{}{name}:",
                        if index > 0 { ";" } else { "" }
                    ))),
                    right: escaped,
                })));
            }
            let mut result = parts.remove(0);
            for part in parts {
                result = Box::new(Expr::Bin(BinExpr {
                    span: DUMMY_SP,
                    op: op!(bin, "+"),
                    left: result,
                    right: part,
                }));
            }
            return result;
        }
        let ssr_style = self.register_import("ssrStyle");
        call(Box::new(Expr::Ident(ssr_style)), vec![arg(expression)])
    }

    fn ssr_class_list(&mut self, expression: Box<Expr>) -> Box<Expr> {
        if let Expr::Object(object) = &*expression
            && !object
                .props
                .iter()
                .any(|prop| matches!(prop, PropOrSpread::Spread(_)))
        {
            let mut values = Vec::new();
            let mut quasis = vec![String::new()];
            self.transform_classlist_object(object.clone(), &mut values, &mut quasis);
            if values.is_empty() {
                return Box::new(str_lit(&quasis[0]));
            }
            if values.len() == 1 && quasis[0].is_empty() && quasis[1].is_empty() {
                return values.remove(0);
            }
            let count = quasis.len();
            return Box::new(Expr::Tpl(Tpl {
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
            }));
        }
        let class_list = self.register_import("ssrClassList");
        call(Box::new(Expr::Ident(class_list)), vec![arg(expression)])
    }

    fn set_attr_ssr(&mut self, results: &mut Results, name: &str, value: Box<Expr>, is_svg: bool) {
        // Namespaces are stripped: at this point everything is an attribute.
        let name = match name.split_once(':') {
            Some((ns, rest)) if !rest.is_empty() && RESERVED_NAMESPACES.contains(&ns) => rest,
            _ => name,
        };
        let name = to_attribute(name, is_svg);
        let ssr_attribute = self.register_import("ssrAttribute");
        let attr = call(
            Box::new(Expr::Ident(ssr_attribute)),
            vec![arg(Box::new(str_lit(&name))), arg(value), bool_arg(false)],
        );
        if results
            .template_parts
            .last()
            .is_some_and(|chunk| !chunk.is_empty())
        {
            results.template_parts.push(String::new());
            results.template_values.push(attr);
        } else {
            let last = results.template_values.len() - 1;
            let previous = results.template_values[last].clone();
            *results.template_values[last] = Expr::Bin(BinExpr {
                span: DUMMY_SP,
                op: op!(bin, "+"),
                left: previous,
                right: attr,
            });
        }
    }
}

// ---------------------------------------------------------------- escaping, children, spreads

impl<C: Comments> Transform<C> {
    /// `escapeExpression`: push the `escape()` call as deep as it will go, so literals and the
    /// static parts of an expression never pay for it at runtime.
    fn escape_expression(
        &mut self,
        expression: Box<Expr>,
        attr: bool,
        escape_literals: bool,
    ) -> Box<Expr> {
        match *expression {
            Expr::Lit(Lit::Str(ref s)) if escape_literals => {
                Box::new(str_lit(&escape_html(&s.value.to_atom_lossy(), attr)))
            }
            Expr::Lit(Lit::Str(_)) | Expr::Lit(Lit::Num(_)) => expression,
            Expr::Tpl(ref tpl) if tpl.exprs.is_empty() => {
                if escape_literals {
                    let raw = tpl.quasis[0].raw.to_string();
                    Box::new(str_lit(&escape_html(&raw, attr)))
                } else {
                    expression
                }
            }
            Expr::Arrow(mut arrow) => {
                match *arrow.body {
                    ArrowFunctionBody::FunctionBody(mut block) => {
                        self.escape_return_statements(&mut block.stmts, attr, escape_literals);
                        arrow.body = Box::new(ArrowFunctionBody::FunctionBody(block));
                    }
                    ArrowFunctionBody::Expr(body) => {
                        arrow.body = Box::new(ArrowFunctionBody::Expr(self.escape_expression(
                            body,
                            attr,
                            escape_literals,
                        )));
                    }
                }
                Box::new(Expr::Arrow(arrow))
            }
            Expr::Fn(mut function) => {
                if let Some(block) = &mut function.function.body {
                    let mut stmts = std::mem::take(&mut block.stmts);
                    self.escape_return_statements(&mut stmts, attr, escape_literals);
                    block.stmts = stmts;
                }
                Box::new(Expr::Fn(function))
            }
            Expr::Tpl(mut tpl) => {
                tpl.exprs = tpl
                    .exprs
                    .into_iter()
                    .map(|expr| self.escape_expression(expr, attr, escape_literals))
                    .collect();
                Box::new(Expr::Tpl(tpl))
            }
            Expr::Unary(_) => expression,
            Expr::Bin(mut bin) if !crate::utils::is_logical(bin.op) => {
                bin.left = self.escape_expression(bin.left, attr, escape_literals);
                bin.right = self.escape_expression(bin.right, attr, escape_literals);
                Box::new(Expr::Bin(bin))
            }
            Expr::Cond(mut cond) => {
                cond.cons = self.escape_expression(cond.cons, attr, escape_literals);
                cond.alt = self.escape_expression(cond.alt, attr, escape_literals);
                Box::new(Expr::Cond(cond))
            }
            Expr::Bin(mut bin) => {
                bin.right = self.escape_expression(bin.right, attr, escape_literals);
                if bin.op != op!("&&") {
                    bin.left = self.escape_expression(bin.left, attr, escape_literals);
                }
                Box::new(Expr::Bin(bin))
            }
            Expr::Call(mut call_expr)
                if matches!(&call_expr.callee, Callee::Expr(callee)
                    if matches!(&**callee, Expr::Fn(_) | Expr::Arrow(_))) =>
            {
                let Callee::Expr(callee) = call_expr.callee else {
                    unreachable!()
                };
                call_expr.callee =
                    Callee::Expr(self.escape_expression(callee, attr, escape_literals));
                Box::new(Expr::Call(call_expr))
            }
            Expr::JSXElement(ref el) if !is_component(&get_tag_name(el)) => {
                self.wont_escape.insert(el.span().lo);
                expression
            }
            _ => {
                let escape = self.register_import("escape");
                let mut args = vec![arg(expression)];
                if attr {
                    args.push(bool_arg(true));
                }
                call(Box::new(Expr::Ident(escape)), args)
            }
        }
    }

    fn escape_return_statements(
        &mut self,
        stmts: &mut Vec<Stmt>,
        attr: bool,
        escape_literals: bool,
    ) {
        for stmt in stmts {
            if let Stmt::Return(ret) = stmt
                && let Some(argument) = ret.arg.take()
            {
                ret.arg = Some(self.escape_expression(argument, attr, escape_literals));
            }
        }
    }

    fn transform_children_ssr(
        &mut self,
        children: Vec<JSXElementChild>,
        results: &mut Results,
        do_not_escape: bool,
    ) {
        let filtered = filter_children(children);
        let multi = check_length(&filtered);
        let markers = self.config.hydratable && multi;
        for child in filtered {
            if let JSXElementChild::JSXElement(el) = &child
                && get_tag_name(el) == "head"
            {
                let hydratable = self.config.hydratable;
                self.config.hydratable = false;
                let transformed = self.transform_node(
                    child,
                    Info {
                        do_not_escape,
                        parent_is_native: true,
                        ..Default::default()
                    },
                );
                self.config.hydratable = hydratable;
                if let Some(transformed) = transformed {
                    let wrapped = self.no_hydration_component(transformed);
                    results.template_parts.push(String::new());
                    results.template_values.push(wrapped);
                }
                continue;
            }
            let Some(mut transformed) = self.transform_node(
                child,
                Info {
                    do_not_escape,
                    parent_is_native: true,
                    ..Default::default()
                },
            ) else {
                continue;
            };
            let chunks = if transformed.template_parts.is_empty() {
                vec![transformed.template.clone()]
            } else {
                transformed.template_parts.clone()
            };
            append_to_template(&mut results.template_parts, &chunks);
            results
                .template_values
                .append(&mut transformed.template_values);
            if !transformed.exprs.is_empty() {
                let spread_element = transformed.spread_element;
                let mut value = transformed.into_first_expr();
                if !do_not_escape && !spread_element {
                    value = self.escape_expression(value, false, false);
                }
                if markers && !spread_element {
                    append_to_template(&mut results.template_parts, &["<!--#-->".into()]);
                    results.template_parts.push(String::new());
                    results.template_values.push(value);
                    append_to_template(&mut results.template_parts, &["<!--/-->".into()]);
                } else {
                    results.template_parts.push(String::new());
                    results.template_values.push(value);
                }
            }
        }
    }
}

impl<C: Comments> Transform<C> {
    /// `createElement`: the spread path, which hands the whole element to `ssrElement` at
    /// runtime instead of building a static template.
    fn create_element_ssr(&mut self, el: JSXElement, info: &Info) -> Results {
        let tag_name = get_tag_name(&el);
        let attributes = self.normalize_attributes(&el);
        let filtered = filter_children(el.children.clone());
        let multi = check_length(&filtered);
        let markers = self.config.hydratable && multi;

        let mut child_nodes: Vec<Box<Expr>> = Vec::new();
        for child in filtered {
            if let JSXElementChild::JSXText(text) = &child {
                let value = decode_html_entities(&trim_whitespace(&text.raw));
                if !value.is_empty() {
                    child_nodes.push(Box::new(str_lit(&value)));
                }
                continue;
            }
            // These children are still inside a native element, which is what
            // `getStaticExpression` reads off the AST parent upstream.
            let info = Info {
                parent_is_native: true,
                ..Default::default()
            };
            let Some(mut transformed) = self.transform_node(child, info) else {
                continue;
            };
            let inline = !transformed.exprs.is_empty() && !transformed.spread_element;
            if markers && inline {
                child_nodes.push(Box::new(str_lit("<!--#-->")));
            }
            if inline {
                let value = std::mem::take(&mut transformed.exprs);
                let Stmt::Expr(stmt) = value.into_iter().next().unwrap() else {
                    unreachable!()
                };
                let escaped = self.escape_expression(stmt.expr, false, false);
                transformed.exprs = vec![expr_stmt(escaped)];
            }
            child_nodes.push(self.create_template_for(transformed, true));
            if markers && inline {
                child_nodes.push(Box::new(str_lit("<!--/-->")));
            }
        }

        let props = self.element_props(attributes, !el.children.is_empty());
        let ssr_element = self.register_import("ssrElement");
        let children_arg = if child_nodes.is_empty() {
            Box::new(Expr::Ident(ident("undefined")))
        } else {
            let collected = if child_nodes.len() == 1 {
                child_nodes.remove(0)
            } else {
                Box::new(Expr::Array(ArrayLit {
                    span: DUMMY_SP,
                    elems: child_nodes
                        .into_iter()
                        .map(|expr| Some(arg(expr)))
                        .collect(),
                }))
            };
            if self.config.hydratable {
                arrow_expr(Vec::new(), collected)
            } else {
                collected
            }
        };
        let expr = call(
            Box::new(Expr::Ident(ssr_element)),
            vec![
                arg(Box::new(str_lit(&tag_name))),
                arg(props),
                arg(children_arg),
                bool_arg(info.top_level && self.config.hydratable),
            ],
        );
        Results {
            spread_element: true,
            ..Results::from_expr(expr)
        }
    }

    fn element_props(&mut self, attributes: Vec<JSXAttrOrSpread>, has_children: bool) -> Box<Expr> {
        if attributes.len() == 1
            && let JSXAttrOrSpread::SpreadElement(spread) = &attributes[0]
        {
            return spread.expr.clone();
        }
        let mut props: Vec<Box<Expr>> = Vec::new();
        let mut running_object: Vec<PropOrSpread> = Vec::new();
        let mut dynamic_spread = false;
        for attribute in attributes {
            match attribute {
                JSXAttrOrSpread::SpreadElement(spread) => {
                    if !running_object.is_empty() {
                        props.push(Box::new(Expr::Object(ObjectLit {
                            span: DUMMY_SP,
                            props: std::mem::take(&mut running_object),
                        })));
                    }
                    let argument = spread.expr;
                    if self.is_dynamic(&argument, DynamicOpts::new().member()) {
                        dynamic_spread = true;
                        props.push(unwrap_zero_arg_call(argument));
                    } else {
                        props.push(argument);
                    }
                }
                JSXAttrOrSpread::JSXAttr(attr) => {
                    let id = convert_jsx_identifier(&attr.name);
                    let key = attr_key(&attr.name);
                    if has_children && key == "children" {
                        continue;
                    }
                    if key == "ref"
                        || key.starts_with("use:")
                        || key.starts_with("prop:")
                        || key.starts_with("on")
                    {
                        continue;
                    }
                    let container = match &attr.value {
                        Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            expr: JSXExpr::Expr(expr),
                            ..
                        })) => Some(expr.clone()),
                        _ => None,
                    };
                    let Some(expression) = container else {
                        let value = match &attr.value {
                            Some(JSXAttrValue::Str(s)) => Box::new(Expr::Lit(Lit::Str(s.clone()))),
                            Some(JSXAttrValue::JSXElement(el)) => {
                                Box::new(Expr::JSXElement(el.clone()))
                            }
                            Some(JSXAttrValue::JSXFragment(fragment)) => {
                                Box::new(Expr::JSXFragment(fragment.clone()))
                            }
                            _ => Box::new(Expr::Lit(Lit::Bool(Bool {
                                span: DUMMY_SP,
                                value: true,
                            }))),
                        };
                        running_object.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                            KeyValueProp { key: id, value },
                        ))));
                        continue;
                    };
                    if self.is_dynamic(&expression, DynamicOpts::new().member().tags()) {
                        let computed =
                            prop_key_name(&id).is_none_or(|name| !is_valid_identifier(name));
                        running_object.push(crate::element::getter(id, computed, expression));
                    } else {
                        running_object.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                            KeyValueProp {
                                key: id,
                                value: expression,
                            },
                        ))));
                    }
                }
            }
        }
        if !running_object.is_empty() || props.is_empty() {
            props.push(Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: running_object,
            })));
        }
        if props.len() > 1 || dynamic_spread {
            let merge = self.register_import("mergeProps");
            return call(
                Box::new(Expr::Ident(merge)),
                props.into_iter().map(arg).collect(),
            );
        }
        props.remove(0)
    }
}
