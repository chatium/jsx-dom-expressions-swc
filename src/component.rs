use swc_core::common::DUMMY_SP;
use swc_core::common::comments::Comments;
use swc_core::ecma::ast::*;

use crate::element::{arrow_body, getter, strip_ts_wrappers, unwrap_zero_arg_call};
use crate::template::{
    arg, arrow_expr, call, expr_to_assign_target, fn_body, getter_fn, member, str_lit,
};
use crate::transform::{Condition, Info, Results, decode_html_entities};
use crate::utils::{
    DynamicOpts, convert_jsx_identifier, filter_children, ident, is_logical, is_valid_identifier,
    prop_key_name, trim_whitespace,
};
use crate::{Generate, Transform};

/// A component's `children` getter body, which is an expression unless the child already
/// carried a block.
enum GetterBody {
    Expr(Box<Expr>),
    Block(FunctionBody),
}

impl<C: Comments> Transform<C> {
    pub(crate) fn transform_component(&mut self, mut el: JSXElement) -> Results {
        let mut tag = convert_component_identifier(&el.opening.name);
        let mut props: Vec<Box<Expr>> = Vec::new();
        let mut running_object: Vec<PropOrSpread> = Vec::new();
        let mut dynamic_spread = false;
        let has_children = !el.children.is_empty();

        if let Expr::Ident(id) = &*tag
            && self.config.built_ins.iter().any(|name| *name == *id.sym)
            && !self.bindings.has_name(&id.sym)
        {
            let imported = self.register_import(id.sym.as_ref());
            tag = Box::new(Expr::Ident(imported));
        }

        for attribute in std::mem::take(&mut el.opening.attrs) {
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
                    let key = convert_jsx_identifier(&attr.name);
                    let key_name = prop_key_name(&key).map(str::to_string);
                    if has_children && key_name.as_deref() == Some("children") {
                        continue;
                    }
                    let computed = key_name
                        .as_deref()
                        .is_none_or(|name| !is_valid_identifier(name));
                    let container_expr = match &attr.value {
                        Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            expr: JSXExpr::Expr(expr),
                            ..
                        })) => Some(expr.clone()),
                        _ => None,
                    };
                    let Some(expression) = container_expr else {
                        // `t.isStringLiteral(node.value) ? t.stringLiteral(node.value.value) : …`
                        // upstream, working around a Babel HTML-entity quirk.
                        let value = match &attr.value {
                            Some(JSXAttrValue::Str(s)) => Box::new(Expr::Lit(Lit::Str(s.clone()))),
                            Some(JSXAttrValue::JSXElement(el)) => {
                                Box::new(Expr::JSXElement(el.clone()))
                            }
                            Some(JSXAttrValue::JSXFragment(fragment)) => {
                                Box::new(Expr::JSXFragment(fragment.clone()))
                            }
                            Some(JSXAttrValue::JSXExprContainer(_)) | None => {
                                Box::new(Expr::Lit(Lit::Bool(Bool {
                                    span: DUMMY_SP,
                                    value: true,
                                })))
                            }
                        };
                        running_object.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                            KeyValueProp { key, value },
                        ))));
                        continue;
                    };

                    if key_name.as_deref() == Some("ref") {
                        if self.config.generate != Generate::Ssr {
                            self.component_ref(expression, &mut running_object);
                        }
                        continue;
                    }

                    if self.is_dynamic(&expression, DynamicOpts::new().member().tags()) {
                        let body = if self.config.wrap_conditionals
                            && self.config.generate != Generate::Ssr
                            && (matches!(&*expression, Expr::Cond(_))
                                || matches!(&*expression, Expr::Bin(bin) if is_logical(bin.op)))
                        {
                            match self.transform_condition(expression, true, false) {
                                Condition::Expr(arrow) => arrow_body(arrow),
                                Condition::Hoisted(..) => unreachable!("inline never hoists"),
                            }
                        } else {
                            expression
                        };
                        running_object.push(getter(key, computed, body));
                    } else {
                        running_object.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                            KeyValueProp {
                                key,
                                value: expression,
                            },
                        ))));
                    }
                }
            }
        }

        if let Some((children, dynamic)) = self.transform_component_children(el.children) {
            if dynamic {
                let body = match &*children {
                    Expr::Call(call)
                        if call.args.first().is_some_and(|arg| {
                            matches!(&*arg.expr, Expr::Fn(_) | Expr::Arrow(_))
                        }) =>
                    {
                        function_body(call.args[0].expr.clone())
                    }
                    Expr::Arrow(_) | Expr::Fn(_) => function_body(children.clone()),
                    _ => GetterBody::Expr(children.clone()),
                };
                running_object.push(PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
                    span: DUMMY_SP,
                    key: PropName::Ident(IdentName::new("children".into(), DUMMY_SP)),
                    function: getter_fn(
                        Vec::new(),
                        match body {
                            GetterBody::Expr(expr) => fn_body(vec![Stmt::Return(ReturnStmt {
                                span: DUMMY_SP,
                                arg: Some(expr),
                            })]),
                            GetterBody::Block(block) => block,
                        },
                    ),
                }))));
            } else {
                running_object.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Ident(IdentName::new("children".into(), DUMMY_SP)),
                    value: children,
                }))));
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
            props = vec![call(
                Box::new(Expr::Ident(merge)),
                props.into_iter().map(arg).collect(),
            )];
        }
        let create_component = self.register_import("createComponent");
        let expr = call(
            Box::new(Expr::Ident(create_component)),
            vec![arg(tag), arg(props.remove(0))],
        );
        Results::from_expr(expr)
    }

    fn component_ref(&mut self, expression: Box<Expr>, running_object: &mut Vec<PropOrSpread>) {
        let expression = strip_ts_wrappers(expression);
        let is_const_fn = matches!(&*expression, Expr::Ident(id)
            if self.bindings.get(id).is_some_and(|binding| binding.is_const));
        if !is_const_fn && is_lval(&expression) {
            let ref_id = self.uid.generate("_ref$");
            running_object.push(ref_method(
                ref_id.clone(),
                expression.clone(),
                Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Cond(CondExpr {
                        span: DUMMY_SP,
                        test: typeof_is_function(&ref_id),
                        cons: call(
                            Box::new(Expr::Ident(ref_id)),
                            vec![arg(Box::new(Expr::Ident(ident("r$"))))],
                        ),
                        alt: Box::new(Expr::Assign(AssignExpr {
                            span: DUMMY_SP,
                            op: op!("="),
                            left: expr_to_assign_target(expression),
                            right: Box::new(Expr::Ident(ident("r$"))),
                        })),
                    })),
                }),
            ));
        } else if is_const_fn || matches!(&*expression, Expr::Fn(_) | Expr::Arrow(_)) {
            running_object.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(IdentName::new("ref".into(), DUMMY_SP)),
                value: expression,
            }))));
        } else if matches!(&*expression, Expr::Call(_)) {
            let ref_id = self.uid.generate("_ref$");
            running_object.push(ref_method(
                ref_id.clone(),
                expression,
                Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Bin(BinExpr {
                        span: DUMMY_SP,
                        op: op!("&&"),
                        left: typeof_is_function(&ref_id),
                        right: call(
                            Box::new(Expr::Ident(ref_id)),
                            vec![arg(Box::new(Expr::Ident(ident("r$"))))],
                        ),
                    })),
                }),
            ));
        }
    }

    fn transform_component_children(
        &mut self,
        children: Vec<JSXElementChild>,
    ) -> Option<(Box<Expr>, bool)> {
        let filtered = filter_children(children);
        if filtered.is_empty() {
            return None;
        }
        let multiple = filtered.len() > 1;
        let mut dynamic = false;
        let mut transformed: Vec<Box<Expr>> = Vec::new();
        let mut kinds: Vec<JSXElementChild> = Vec::new();

        for child in filtered {
            if let JSXElementChild::JSXText(text) = &child {
                let value = decode_html_entities(&trim_whitespace(&text.raw));
                if !value.is_empty() {
                    kinds.push(child.clone());
                    transformed.push(Box::new(str_lit(&value)));
                }
                continue;
            }
            let info = Info {
                top_level: true,
                component_child: true,
                last_element: true,
                ..Default::default()
            };
            let Some(result) = self.transform_node(child.clone(), info) else {
                continue;
            };
            dynamic = dynamic || result.dynamic;
            kinds.push(child);
            let mut result = result;
            // Under SSR a dynamic sibling is inlined rather than kept as a thunk.
            if self.config.generate == Generate::Ssr
                && multiple
                && result.dynamic
                && result.id.is_none()
                && result.exprs.len() == 1
                && let Stmt::Expr(stmt) = &result.exprs[0]
                && matches!(&*stmt.expr, Expr::Fn(_) | Expr::Arrow(_))
            {
                let inner = crate::element::arrow_body(stmt.expr.clone());
                result.exprs = vec![crate::template::expr_stmt(inner)];
            }
            let expr = self.create_template_for(result, multiple);
            transformed.push(expr);
        }

        if transformed.is_empty() {
            return None;
        }

        if transformed.len() == 1 {
            let mut only = transformed.remove(0);
            let wraps = !matches!(
                kinds[0],
                JSXElementChild::JSXExprContainer(_)
                    | JSXElementChild::JSXSpreadChild(_)
                    | JSXElementChild::JSXText(_)
            );
            if wraps {
                only = match &*only {
                    Expr::Call(call)
                        if call.args.is_empty()
                            && matches!(&call.callee, Callee::Expr(callee)
                                if !matches!(&**callee, Expr::Ident(_))) =>
                    {
                        let Callee::Expr(callee) = call.callee.clone() else {
                            unreachable!()
                        };
                        callee
                    }
                    _ => arrow_expr(Vec::new(), only),
                };
                dynamic = true;
            }
            Some((only, dynamic))
        } else {
            Some((
                arrow_expr(
                    Vec::new(),
                    Box::new(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: transformed
                            .into_iter()
                            .map(|expr| Some(ExprOrSpread { spread: None, expr }))
                            .collect(),
                    })),
                ),
                true,
            ))
        }
    }
}

fn convert_component_identifier(name: &JSXElementName) -> Box<Expr> {
    match name {
        JSXElementName::Ident(id) if is_valid_identifier(&id.sym) => {
            Box::new(Expr::Ident(ident(&id.sym)))
        }
        JSXElementName::Ident(id) => Box::new(str_lit(&id.sym)),
        JSXElementName::JSXMemberExpr(expr) => convert_jsx_member(expr),
        JSXElementName::JSXNamespacedName(ns) => {
            Box::new(str_lit(&format!("{}:{}", ns.ns.sym, ns.name.sym)))
        }
    }
}

fn convert_jsx_member(expr: &JSXMemberExpr) -> Box<Expr> {
    let object = match &expr.obj {
        JSXObject::Ident(id) if is_valid_identifier(&id.sym) => {
            Box::new(Expr::Ident(ident(&id.sym)))
        }
        JSXObject::Ident(id) => Box::new(str_lit(&id.sym)),
        JSXObject::JSXMemberExpr(inner) => convert_jsx_member(inner),
    };
    if is_valid_identifier(&expr.prop.sym) {
        member(object, &expr.prop.sym)
    } else {
        Box::new(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: object,
            prop: MemberProp::Computed(ComputedPropName {
                span: DUMMY_SP,
                expr: Box::new(str_lit(&expr.prop.sym)),
            }),
        }))
    }
}

fn ref_method(ref_id: Ident, init: Box<Expr>, statement: Stmt) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::Method(MethodProp {
        key: PropName::Ident(IdentName::new("ref".into(), DUMMY_SP)),
        function: getter_fn(
            vec![Param {
                span: DUMMY_SP,
                decorators: Vec::new(),
                pat: Pat::Ident(ident("r$").into()),
            }],
            fn_body(vec![
                Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Const,
                    declare: false,
                    decls: vec![VarDeclarator {
                        span: DUMMY_SP,
                        name: Pat::Ident(ref_id.into()),
                        init: Some(init),
                        definite: false,
                    }],
                    ..Default::default()
                }))),
                statement,
            ]),
        ),
    })))
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

// Takes the box because it unwraps into bodies the AST already owns.
#[allow(clippy::boxed_local)]
fn function_body(expr: Box<Expr>) -> GetterBody {
    match *expr {
        Expr::Arrow(arrow) => match *arrow.body {
            ArrowFunctionBody::Expr(expr) => GetterBody::Expr(expr),
            ArrowFunctionBody::FunctionBody(block) => GetterBody::Block(block),
        },
        Expr::Fn(function) => match function.function.body {
            Some(block) => GetterBody::Block(block),
            None => GetterBody::Expr(Box::new(Expr::Fn(function))),
        },
        other => GetterBody::Expr(Box::new(other)),
    }
}

fn is_lval(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Ident(_) | Expr::Member(_) | Expr::Array(_) | Expr::Object(_) | Expr::SuperProp(_)
    )
}
