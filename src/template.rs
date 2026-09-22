use swc_core::common::comments::Comments;
use swc_core::common::{DUMMY_SP, Span};
use swc_core::ecma::ast::*;

use crate::Transform;
use crate::transform::{Dynamic, Results};
use crate::utils::ident;

pub(crate) struct TemplateDef {
    pub(crate) id: Ident,
    pub(crate) template: String,
    pub(crate) is_svg: bool,
    pub(crate) is_ce: bool,
}

impl<C: Comments> Transform<C> {
    /// `registerImportMethod`. With only the DOM renderer there is one module, so the
    /// renderer lookup upstream does collapses to the configured `moduleName`.
    pub(crate) fn register_import(&mut self, name: &str) -> Ident {
        if let Some((_, id)) = self.imports.iter().find(|(key, _)| key == name) {
            return id.clone();
        }
        let id = self.uid.generate(&format!("_${name}"));
        self.imports.push((name.to_string(), id.clone()));
        id
    }

    /// The generated imports and the template declarations, in the order Babel's block
    /// hoisting settles on: every generated import first (newest first, because each one is
    /// inserted at the top), then the templates, then the original body.
    pub(crate) fn module_prologue(&mut self) -> Vec<ModuleItem> {
        let mut out: Vec<ModuleItem> = Vec::new();
        let templates = std::mem::take(&mut self.templates);
        let template_decl = if templates.is_empty() {
            self.append_templates_ssr()
        } else {
            Some(self.append_templates(templates))
        };

        for (name, local) in self.imports.iter().rev() {
            out.push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: local.clone(),
                    imported: Some(ModuleExportName::Ident(ident(name))),
                    is_type_only: false,
                })],
                src: Box::new(Str {
                    span: DUMMY_SP,
                    value: self.config.module_name.clone().into(),
                    raw: None,
                }),
                type_only: false,
                with: None,
                phase: Default::default(),
            })));
        }
        if let Some(decl) = template_decl {
            out.push(ModuleItem::Stmt(decl));
        }
        out
    }

    fn append_templates(&mut self, templates: Vec<TemplateDef>) -> Stmt {
        let template_fn = self.register_import("template");
        let decls = templates
            .into_iter()
            .map(|template| {
                // The text lands in a template literal, and JS normalises line terminators
                // inside those, so emit what a parser would read back.
                let template_text = template.template.replace("\r\n", "\n").replace('\r', "\n");
                let span = pure_span(&self.comments);
                let mut args = vec![ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Tpl(Tpl {
                        span: DUMMY_SP,
                        exprs: Vec::new(),
                        quasis: vec![TplElement {
                            span: DUMMY_SP,
                            tail: true,
                            cooked: Some(template_text.clone().into()),
                            raw: template_text.into(),
                        }],
                    })),
                }];
                if template.is_svg || template.is_ce {
                    args.push(bool_arg(template.is_ce));
                    args.push(bool_arg(template.is_svg));
                }
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(template.id.into()),
                    init: Some(Box::new(Expr::Call(CallExpr {
                        span,
                        callee: Callee::Expr(Box::new(Expr::Ident(template_fn.clone()))),
                        args,
                        ..Default::default()
                    }))),
                    definite: false,
                }
            })
            .collect();
        Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Const,
            declare: false,
            decls,
            ..Default::default()
        })))
    }

    /// `postprocess`: the `delegateEvents` call that closes the module.
    pub(crate) fn module_epilogue(&mut self) -> Vec<Stmt> {
        if self.events.is_empty() {
            return Vec::new();
        }
        let events = std::mem::take(&mut self.events);
        let delegate = self.register_import("delegateEvents");
        vec![Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Ident(delegate))),
                args: vec![ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: events
                            .into_iter()
                            .map(|event| {
                                Some(ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(str_lit(&event)),
                                })
                            })
                            .collect(),
                    })),
                }],
                ..Default::default()
            })),
        })]
    }

    /// `createTemplate` for the DOM generator.
    pub(crate) fn create_template(&mut self, mut result: Results, wrap: bool) -> Box<Expr> {
        if result.id.is_some() {
            self.register_template(&mut result);
            let decl = result.decl.take().expect("registerTemplate sets decl");
            let single_declarator = matches!(
                &decl,
                Stmt::Decl(Decl::Var(var)) if var.decls.len() == 1
            );
            if result.exprs.is_empty()
                && result.dynamics.is_empty()
                && result.post_exprs.is_empty()
                && single_declarator
            {
                let Stmt::Decl(Decl::Var(var)) = decl else {
                    unreachable!()
                };
                return var.decls.into_iter().next().unwrap().init.unwrap();
            }
            let id = result.id.clone().unwrap();
            let mut body = vec![decl];
            body.extend(result.exprs);
            if let Some(effect) = self.wrap_dynamics(std::mem::take(&mut result.dynamics)) {
                body.push(effect);
            }
            body.extend(result.post_exprs);
            body.push(Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(Box::new(Expr::Ident(id))),
            }));
            return call(arrow_block(Vec::new(), body), Vec::new());
        }
        if wrap && result.dynamic && !self.config.memo_wrapper.is_empty() {
            let memo = self.register_import(&self.config.memo_wrapper.clone());
            return call(
                Box::new(Expr::Ident(memo)),
                vec![arg(result.into_first_expr())],
            );
        }
        result.into_first_expr()
    }

    fn register_template(&mut self, results: &mut Results) {
        let mut decl = None;
        if !results.template.is_empty() {
            let mut template_id = None;
            if !results.skip_template {
                template_id = Some(
                    match self
                        .templates
                        .iter()
                        .find(|def| def.template == results.template)
                    {
                        Some(def) => def.id.clone(),
                        None => {
                            let id = self.uid.generate("tmpl$");
                            self.templates.push(TemplateDef {
                                id: id.clone(),
                                template: results.template.clone(),
                                is_svg: results.is_svg,
                                is_ce: results.has_custom_element,
                            });
                            id
                        }
                    },
                );
            }
            let init = if self.config.hydratable {
                let get_next_element = self.register_import("getNextElement");
                call(
                    Box::new(Expr::Ident(get_next_element)),
                    template_id
                        .map(|id| vec![arg(Box::new(Expr::Ident(id)))])
                        .unwrap_or_default(),
                )
            } else {
                call(
                    Box::new(Expr::Ident(
                        template_id.expect("skipTemplate is hydratable-only"),
                    )),
                    Vec::new(),
                )
            };
            decl = Some(VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(results.id.clone().unwrap().into()),
                init: Some(init),
                definite: false,
            });
        }
        // An element always carries a template, so the declarator slot Babel keeps for the
        // empty case never comes up here.
        let mut decls = Vec::new();
        decls.extend(decl);
        decls.append(&mut results.declarations);
        results.decl = Some(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Const,
            declare: false,
            decls,
            ..Default::default()
        }))));
    }

    fn wrap_dynamics(&mut self, dynamics: Vec<Dynamic>) -> Option<Stmt> {
        if dynamics.is_empty() {
            return None;
        }
        let effect = self.register_import(&self.config.effect_wrapper.clone());

        if dynamics.len() == 1 {
            let mut dynamic = dynamics.into_iter().next().unwrap();
            let prev_value =
                (dynamic.key == "classList" || dynamic.key == "style").then(|| ident("_$p"));
            if dynamic.key.starts_with("class:")
                && !matches!(&*dynamic.value, Expr::Lit(Lit::Bool(_)) | Expr::Unary(_))
            {
                dynamic.value = double_bang(dynamic.value);
            }
            let body = self.set_attr(
                Box::new(Expr::Ident(dynamic.elem.clone())),
                &dynamic.key,
                dynamic.value,
                crate::element::SetAttrOpts {
                    is_svg: dynamic.is_svg,
                    is_ce: dynamic.is_ce,
                    tag_name: dynamic.tag_name.clone(),
                    dynamic: true,
                    prev_id: prev_value.clone().map(|id| Box::new(Expr::Ident(id))),
                },
            );
            let params: Vec<Pat> = prev_value
                .into_iter()
                .map(|id| Pat::Ident(id.into()))
                .collect();
            return Some(expr_stmt(call(
                Box::new(Expr::Ident(effect)),
                vec![arg(arrow_expr(params, body))],
            )));
        }

        let prev_id = ident("_p$");
        let mut declarations = Vec::new();
        let mut statements = Vec::new();
        let mut identifiers = Vec::new();
        for mut dynamic in dynamics {
            let identifier = self.uid.generate("v$");
            if dynamic.key.starts_with("class:")
                && !matches!(&*dynamic.value, Expr::Lit(Lit::Bool(_)) | Expr::Unary(_))
            {
                dynamic.value = double_bang(dynamic.value);
            }
            identifiers.push(identifier.clone());
            declarations.push(VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(identifier.clone().into()),
                init: Some(dynamic.value),
                definite: false,
            });
            let prev = member(
                Box::new(Expr::Ident(prev_id.clone())),
                identifier.sym.as_str(),
            );
            if dynamic.key == "classList" || dynamic.key == "style" {
                let value = self.set_attr(
                    Box::new(Expr::Ident(dynamic.elem.clone())),
                    &dynamic.key,
                    Box::new(Expr::Ident(identifier.clone())),
                    crate::element::SetAttrOpts {
                        is_svg: dynamic.is_svg,
                        is_ce: dynamic.is_ce,
                        tag_name: dynamic.tag_name.clone(),
                        dynamic: true,
                        prev_id: Some(prev.clone()),
                    },
                );
                statements.push(expr_stmt(Box::new(Expr::Assign(AssignExpr {
                    span: DUMMY_SP,
                    op: op!("="),
                    left: expr_to_assign_target(prev),
                    right: value,
                }))));
            } else {
                let prev_for_set = dynamic
                    .key
                    .starts_with("style:")
                    .then(|| Box::new(Expr::Ident(identifier.clone())));
                let assign = Box::new(Expr::Assign(AssignExpr {
                    span: DUMMY_SP,
                    op: op!("="),
                    left: expr_to_assign_target(prev.clone()),
                    right: Box::new(Expr::Ident(identifier.clone())),
                }));
                let value = self.set_attr(
                    Box::new(Expr::Ident(dynamic.elem.clone())),
                    &dynamic.key,
                    assign,
                    crate::element::SetAttrOpts {
                        is_svg: dynamic.is_svg,
                        is_ce: dynamic.is_ce,
                        tag_name: dynamic.tag_name.clone(),
                        dynamic: true,
                        prev_id: prev_for_set,
                    },
                );
                statements.push(expr_stmt(Box::new(Expr::Bin(BinExpr {
                    span: DUMMY_SP,
                    op: op!("&&"),
                    left: Box::new(Expr::Bin(BinExpr {
                        span: DUMMY_SP,
                        op: op!("!=="),
                        left: Box::new(Expr::Ident(identifier.clone())),
                        right: prev,
                    })),
                    right: value,
                }))));
            }
        }

        let mut body = vec![Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Const,
            declare: false,
            decls: declarations,
            ..Default::default()
        })))];
        body.extend(statements);
        body.push(Stmt::Return(ReturnStmt {
            span: DUMMY_SP,
            arg: Some(Box::new(Expr::Ident(prev_id.clone()))),
        }));

        Some(expr_stmt(call(
            Box::new(Expr::Ident(effect)),
            vec![
                arg(arrow_block(vec![Pat::Ident(prev_id.into())], body)),
                arg(Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: identifiers
                        .into_iter()
                        .map(|id| {
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(IdentName::new(id.sym, DUMMY_SP)),
                                value: Box::new(Expr::Ident(ident("undefined"))),
                            })))
                        })
                        .collect(),
                }))),
            ],
        )))
    }
}

// ---------------------------------------------------------------- small builders

fn pure_span<C: Comments>(comments: &C) -> Span {
    let span = Span::dummy_with_cmt();
    comments.add_pure_comment(span.lo);
    span
}

pub(crate) fn bool_arg(value: bool) -> ExprOrSpread {
    arg(Box::new(Expr::Lit(Lit::Bool(Bool {
        span: DUMMY_SP,
        value,
    }))))
}

pub(crate) fn arg(expr: Box<Expr>) -> ExprOrSpread {
    ExprOrSpread { spread: None, expr }
}

pub(crate) fn str_lit(value: &str) -> Expr {
    Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: value.into(),
        raw: None,
    }))
}

pub(crate) fn call(callee: Box<Expr>, args: Vec<ExprOrSpread>) -> Box<Expr> {
    Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(callee),
        args,
        ..Default::default()
    }))
}

pub(crate) fn member(obj: Box<Expr>, prop: &str) -> Box<Expr> {
    Box::new(Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj,
        prop: MemberProp::Ident(IdentName::new(prop.into(), DUMMY_SP)),
    }))
}

pub(crate) fn expr_stmt(expr: Box<Expr>) -> Stmt {
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr,
    })
}

pub(crate) fn arrow_expr(params: Vec<Pat>, body: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        params,
        body: Box::new(ArrowFunctionBody::Expr(body)),
        is_async: false,
        is_generator: false,
        ..Default::default()
    }))
}

pub(crate) fn arrow_block(params: Vec<Pat>, body: Vec<Stmt>) -> Box<Expr> {
    Box::new(Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        params,
        body: Box::new(ArrowFunctionBody::FunctionBody(fn_body(body))),
        is_async: false,
        is_generator: false,
        ..Default::default()
    }))
}

pub(crate) fn fn_body(stmts: Vec<Stmt>) -> FunctionBody {
    FunctionBody {
        span: DUMMY_SP,
        stmts,
    }
}

/// A zero-parameter function for a getter, or one taking `r$` for a component ref.
pub(crate) fn getter_fn(params: Vec<Param>, body: FunctionBody) -> Box<Function> {
    Box::new(Function {
        params,
        decorators: Vec::new(),
        span: DUMMY_SP,
        body: Some(body),
        is_generator: false,
        is_async: false,
        ..Default::default()
    })
}

pub(crate) fn double_bang(expr: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Unary(UnaryExpr {
        span: DUMMY_SP,
        op: op!("!"),
        arg: Box::new(Expr::Unary(UnaryExpr {
            span: DUMMY_SP,
            op: op!("!"),
            arg: expr,
        })),
    }))
}

// Takes the box because every caller already holds one.
#[allow(clippy::boxed_local)]
pub(crate) fn expr_to_assign_target(expr: Box<Expr>) -> AssignTarget {
    match *expr {
        Expr::Ident(ident) => AssignTarget::Simple(SimpleAssignTarget::Ident(ident.into())),
        Expr::Member(member) => AssignTarget::Simple(SimpleAssignTarget::Member(member)),
        other => AssignTarget::Simple(SimpleAssignTarget::Paren(ParenExpr {
            span: DUMMY_SP,
            expr: Box::new(other),
        })),
    }
}
