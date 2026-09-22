use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

use swc_core::atoms::Atom;
use swc_core::common::BytePos;
use swc_core::common::Spanned;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

/// What the plugin needs out of Babel's scope: whether a name is bound at all, whether a
/// binding is `const`, and what a `const`/`var` binding was initialised with.
#[derive(Debug, Clone)]
pub(crate) struct Binding {
    pub(crate) is_const: bool,
    /// Set for a variable declarator; `None` for a function, class, import or parameter.
    pub(crate) init: Option<Box<Expr>>,
    /// A function declaration resolves an event handler without an initialiser.
    pub(crate) is_function_decl: bool,
    /// Babel's `constantViolations`: an assignment after the declaration stops `evaluate()`
    /// from folding through the binding.
    pub(crate) reassigned: bool,
    /// End of the declarator. Babel deopts a reference that starts before it — a hoisted use
    /// of a `const` declared further down must not fold.
    pub(crate) decl_end: BytePos,
    /// How many times the binding is referenced. Babel's `path.resolve()` gives up past one,
    /// since it cannot tell whether the value was mutated between uses.
    pub(crate) references: usize,
}

#[derive(Debug, Default)]
pub(crate) struct Bindings {
    by_id: HashMap<Id, Binding>,
    names: HashSet<Atom>,
    /// Every name the program mentions, which is what `generateUid` must avoid. Collected
    /// here because the walk that finds bindings already visits each identifier.
    pub(crate) taken: HashSet<Atom>,
    violations: Vec<Id>,
    references: HashMap<Id, usize>,
    /// Nesting validation rides along on this walk rather than making its own.
    validate: Option<crate::Errors>,
}

impl Bindings {
    /// One walk feeds all of it: the bindings themselves, the assignments that deopt
    /// `evaluate()`, the reference counts it consults, and the set of names `generateUid`
    /// has to avoid. These used to be four separate traversals of the whole program.
    pub(crate) fn from_program(program: &Program, validate: Option<crate::Errors>) -> Self {
        let mut collector = Bindings {
            validate,
            ..Default::default()
        };
        program.visit_with(&mut collector);
        let violations = std::mem::take(&mut collector.violations);
        for id in violations {
            if let Some(binding) = collector.by_id.get_mut(&id) {
                binding.reassigned = true;
            }
        }
        let references = std::mem::take(&mut collector.references);
        for (id, count) in references {
            if let Some(binding) = collector.by_id.get_mut(&id) {
                binding.references = count;
            }
        }
        collector
    }

    pub(crate) fn get(&self, ident: &Ident) -> Option<&Binding> {
        self.by_id.get(&ident.to_id())
    }

    /// `scope.hasBinding(name)`.
    ///
    /// ponytail: name-only, ignoring which scope the name is bound in. It decides one thing —
    /// whether a `builtIns` component name is shadowed by user code — and a file that both
    /// imports `Show` from the runtime and binds `Show` in an unrelated function is not a
    /// case worth a full scope walk. Use `Id` lookups if that ever bites.
    pub(crate) fn has_name(&self, name: &str) -> bool {
        self.names.contains(&Atom::from(name))
    }

    fn insert(&mut self, ident: &Ident, binding: Binding) {
        self.names.insert(ident.sym.clone());
        self.by_id.insert(ident.to_id(), binding);
    }

    fn insert_pat(&mut self, pat: &Pat, is_const: bool, init: Option<&Expr>, decl_end: BytePos) {
        match pat {
            Pat::Ident(ident) => self.insert(
                &ident.id,
                Binding {
                    is_const,
                    init: init.map(|expr| Box::new(expr.clone())),
                    is_function_decl: false,
                    reassigned: false,
                    decl_end,
                    references: 0,
                },
            ),
            // A destructured binding never carries a usable initialiser.
            other => {
                let mut names = Vec::new();
                collect_pat_idents(other, &mut names);
                for ident in names {
                    self.insert(
                        &ident,
                        Binding {
                            is_const,
                            init: None,
                            is_function_decl: false,
                            reassigned: false,
                            decl_end,
                            references: 0,
                        },
                    );
                }
            }
        }
    }
}

fn collect_pat_idents(pat: &Pat, out: &mut Vec<Ident>) {
    struct Collect<'a>(&'a mut Vec<Ident>);
    impl Visit for Collect<'_> {
        fn visit_binding_ident(&mut self, node: &BindingIdent) {
            self.0.push(node.id.clone());
        }
        fn visit_expr(&mut self, _: &Expr) {
            // Default values are expressions, not bindings.
        }
    }
    pat.visit_with(&mut Collect(out));
}

impl Visit for Bindings {
    /// `validate`, ported from babel-plugin-validate-jsx-nesting by way of upstream. Upstream
    /// reads `path.parent`, so only an element directly inside another element is checked.
    fn visit_jsx_element(&mut self, node: &JSXElement) {
        if let Some(errors) = &self.validate
            && let JSXElementName::Ident(parent) = &node.opening.name
            && !crate::utils::is_component(&parent.sym)
        {
            for child in &node.children {
                let JSXElementChild::JSXElement(child) = child else {
                    continue;
                };
                let JSXElementName::Ident(tag) = &child.opening.name else {
                    continue;
                };
                if !crate::utils::is_component(&tag.sym)
                    && !crate::nesting::is_valid_html_nesting(&parent.sym, &tag.sym)
                {
                    errors.push(
                        child.span,
                        format!(
                            "Invalid JSX: <{}> cannot be child of <{}>",
                            tag.sym, parent.sym
                        ),
                    );
                }
            }
        }
        node.visit_children_with(self);
    }

    /// Every identifier occupies a name, and every one that is not a declaration site is also
    /// a reference. The declaration sites below record the name without the reference.
    fn visit_ident(&mut self, node: &Ident) {
        self.taken.insert(node.sym.clone());
        *self.references.entry(node.to_id()).or_default() += 1;
    }

    fn visit_binding_ident(&mut self, node: &BindingIdent) {
        self.taken.insert(node.id.sym.clone());
        node.type_ann.visit_with(self);
    }

    fn visit_assign_expr(&mut self, node: &AssignExpr) {
        if let Some(ident) = node.left.as_ident() {
            self.violations.push(ident.id.to_id());
        }
        node.visit_children_with(self);
    }

    fn visit_update_expr(&mut self, node: &UpdateExpr) {
        if let Expr::Ident(ident) = &*node.arg {
            self.violations.push(ident.to_id());
        }
        node.visit_children_with(self);
    }

    fn visit_var_decl(&mut self, node: &VarDecl) {
        let is_const = node.kind == VarDeclKind::Const;
        for declarator in &node.decls {
            self.insert_pat(
                &declarator.name,
                is_const,
                declarator.init.as_deref(),
                declarator.span.hi,
            );
        }
        node.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, node: &FnDecl) {
        self.taken.insert(node.ident.sym.clone());
        self.insert(
            &node.ident,
            Binding {
                is_const: false,
                init: None,
                is_function_decl: true,
                reassigned: false,
                decl_end: node.function.span.hi,
                references: 0,
            },
        );
        node.function.visit_with(self);
    }

    fn visit_class_decl(&mut self, node: &ClassDecl) {
        self.taken.insert(node.ident.sym.clone());
        self.insert(
            &node.ident,
            Binding {
                is_const: false,
                init: None,
                is_function_decl: false,
                reassigned: false,
                decl_end: BytePos(0),
                references: 0,
            },
        );
        node.class.visit_with(self);
    }

    fn visit_param(&mut self, node: &Param) {
        self.insert_pat(&node.pat, false, None, node.span.hi);
        node.visit_children_with(self);
    }

    fn visit_pat(&mut self, node: &Pat) {
        // Arrow parameters and catch clauses arrive here rather than through `Param`.
        node.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, node: &ArrowExpr) {
        for pat in &node.params {
            self.insert_pat(pat, false, None, pat.span().hi);
        }
        node.visit_children_with(self);
    }

    fn visit_import_named_specifier(&mut self, node: &ImportNamedSpecifier) {
        // The remote name of an import is not a reference and does not occupy a local name.
        self.taken.insert(node.local.sym.clone());
        self.insert(
            &node.local,
            Binding {
                is_const: false,
                init: None,
                is_function_decl: false,
                reassigned: false,
                decl_end: BytePos(0),
                references: 0,
            },
        );
    }

    fn visit_import_default_specifier(&mut self, node: &ImportDefaultSpecifier) {
        // The remote name of an import is not a reference and does not occupy a local name.
        self.taken.insert(node.local.sym.clone());
        self.insert(
            &node.local,
            Binding {
                is_const: false,
                init: None,
                is_function_decl: false,
                reassigned: false,
                decl_end: BytePos(0),
                references: 0,
            },
        );
    }

    fn visit_import_star_as_specifier(&mut self, node: &ImportStarAsSpecifier) {
        // The remote name of an import is not a reference and does not occupy a local name.
        self.taken.insert(node.local.sym.clone());
        self.insert(
            &node.local,
            Binding {
                is_const: false,
                init: None,
                is_function_decl: false,
                reassigned: false,
                decl_end: BytePos(0),
                references: 0,
            },
        );
    }
}
