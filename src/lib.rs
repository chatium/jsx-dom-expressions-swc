//! A port of `babel-plugin-jsx-dom-expressions` to SWC, scoped to the DOM generator.
//!
//! Run SWC's `resolver` before this pass: binding-sensitive branches (component refs,
//! resolvable event handlers) look bindings up by `Id`, and degrade to name-only
//! matching without it.

mod component;
mod config;
mod constants;
mod element;
mod entities;
mod errors;
mod eval;
mod nesting;
mod preprocess;
mod scope;
mod ssr;
mod template;
mod transform;
mod uid;
mod utils;

use swc_core::common::comments::Comments;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith, visit_mut_pass};

pub use config::{Config, Generate};
pub use errors::{CompileError, Errors};

use scope::Bindings;
use template::TemplateDef;
use uid::UidGen;

/// The pass. `comments` must be the same store the emitter reads, so that the
/// `/*#__PURE__*/` annotations on template calls survive to the output.
///
/// Where the Babel plugin throws — invalid HTML nesting, a fragment nested inside an element —
/// this records a [`CompileError`] in `errors` and keeps going, so check `errors` before
/// using the output.
pub fn jsx_dom_expressions<C: Comments>(config: Config, comments: C, errors: Errors) -> impl Pass {
    visit_mut_pass(Transform {
        config,
        comments,
        errors,
        uid: UidGen::default(),
        bindings: Bindings::default(),
        imports: Vec::new(),
        events: Vec::new(),
        templates: Vec::new(),
        pending: Vec::new(),
        ssr_templates: Vec::new(),
        wont_escape: Default::default(),
    })
}

/// `"use client"` and friends must stay at the top. Babel keeps directives in a separate AST
/// field so they cannot be displaced; SWC makes them ordinary leading statements, so the
/// generated imports have to be spliced in after them.
fn directive_prologue_len(body: &[ModuleItem]) -> usize {
    body.iter()
        .take_while(|item| {
            matches!(
                item,
                ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. }))
                    if matches!(&**expr, Expr::Lit(Lit::Str(_)))
            )
        })
        .count()
}

pub(crate) struct Transform<C: Comments> {
    pub(crate) config: Config,
    pub(crate) comments: C,
    pub(crate) errors: Errors,
    pub(crate) uid: UidGen,
    pub(crate) bindings: Bindings,
    /// Registered imports in registration order, keyed by `module:name`.
    pub(crate) imports: Vec<(String, Ident)>,
    /// Delegated event names, in first-seen order.
    pub(crate) events: Vec<String>,
    pub(crate) templates: Vec<TemplateDef>,
    /// Statements to splice in before the statement currently being visited.
    pub(crate) pending: Vec<Stmt>,
    /// SSR templates, in registration order: `const _tmpl$ = "…"` or an array of chunks.
    pub(crate) ssr_templates: Vec<(Ident, Vec<String>)>,
    /// `node.wontEscape`, which upstream stashes on the JSX node itself: the elements an
    /// enclosing expression has already decided not to escape, keyed by source position.
    pub(crate) wont_escape: std::collections::HashSet<swc_core::common::BytePos>,
}

impl<C: Comments> VisitMut for Transform<C> {
    fn visit_mut_program(&mut self, program: &mut Program) {
        self.uid = UidGen::from_program(program);
        self.bindings = Bindings::from_program(program);
        if self.config.validate {
            self.validate_nesting(program);
        }
        if !self.should_process(program) {
            return;
        }
        program.visit_mut_children_with(self);

        // `postprocess` appends the delegateEvents call first, so its import registers before
        // the template import and therefore prints after it.
        let epilogue = self.module_epilogue();
        let prologue = self.module_prologue();
        match program {
            Program::Module(module) => {
                let at = directive_prologue_len(&module.body);
                module.body.splice(at..at, prologue);
                module
                    .body
                    .extend(epilogue.into_iter().map(ModuleItem::Stmt));
            }
            Program::Script(script) => {
                // The generated imports make this a module, exactly as Babel treats it.
                let stmts: Vec<ModuleItem> = std::mem::take(&mut script.body)
                    .into_iter()
                    .map(ModuleItem::Stmt)
                    .collect();
                let at = directive_prologue_len(&stmts);
                let mut body: Vec<ModuleItem> = Vec::with_capacity(stmts.len() + prologue.len());
                body.extend(stmts[..at].iter().cloned());
                body.extend(prologue);
                body.extend(stmts[at..].iter().cloned());
                body.extend(epilogue.into_iter().map(ModuleItem::Stmt));
                *program = Program::Module(Module {
                    span: script.span,
                    body,
                    shebang: script.shebang.take(),
                });
            }
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::JSXElement(_) | Expr::JSXFragment(_) => {
                let taken = std::mem::replace(
                    expr,
                    Expr::Invalid(Invalid {
                        span: Default::default(),
                    }),
                );
                *expr = self.transform_jsx(taken);
                // Babel re-queues the replacement, so any JSX left inside it is transformed too.
                expr.visit_mut_with(self);
            }
            _ => expr.visit_mut_children_with(self),
        }
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        // A nested block must not flush statements queued for an enclosing one — Babel inserts
        // before `getStatementParent()`, which is the statement the JSX literally sits in.
        let outer = std::mem::take(&mut self.pending);
        let mut out = Vec::with_capacity(stmts.len());
        for mut stmt in stmts.drain(..) {
            stmt.visit_mut_with(self);
            out.append(&mut self.pending);
            out.push(stmt);
        }
        *stmts = out;
        self.pending = outer;
    }

    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        let outer = std::mem::take(&mut self.pending);
        let mut out = Vec::with_capacity(items.len());
        for mut item in items.drain(..) {
            item.visit_mut_with(self);
            out.extend(self.pending.drain(..).map(ModuleItem::Stmt));
            out.push(item);
        }
        *items = out;
        self.pending = outer;
    }
}
