use swc_core::common::Spanned;
use swc_core::common::comments::Comments;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::Transform;
use crate::nesting::is_valid_html_nesting;
use crate::utils::is_component;

impl<C: Comments> Transform<C> {
    /// `requireImportSource`: a file that does not name the library in `@jsxImportSource` is
    /// left alone entirely.
    pub(crate) fn should_process(&self, program: &Program) -> bool {
        let Some(library) = &self.config.require_import_source else {
            return true;
        };
        let names = |comments: Option<Vec<swc_core::common::comments::Comment>>| {
            comments.is_some_and(|comments| {
                comments.iter().any(|comment| {
                    comment
                        .text
                        .find("@jsxImportSource")
                        .is_some_and(|index| comment.text[index..].contains(library.as_str()))
                })
            })
        };
        if names(self.comments.get_leading(program.span().lo))
            || names(self.comments.get_trailing(program.span().lo))
        {
            return true;
        }
        // A file-level pragma usually sits above the first statement rather than the program.
        match program {
            Program::Module(module) => module
                .body
                .iter()
                .any(|item| names(self.comments.get_leading(item.span().lo))),
            Program::Script(script) => script
                .body
                .iter()
                .any(|item| names(self.comments.get_leading(item.span().lo))),
        }
    }

    /// The `validate` option, ported from babel-plugin-validate-jsx-nesting by way of upstream.
    ///
    /// Upstream reads `path.parent`, so only an element that is *directly* a child of another
    /// element is checked: one behind an expression container, an attribute or an intervening
    /// element has a different AST parent and is left alone.
    pub(crate) fn validate_nesting(&self, program: &Program) {
        struct Validator<'a, C: Comments>(&'a Transform<C>);
        impl<C: Comments> Visit for Validator<'_, C> {
            fn visit_jsx_element(&mut self, node: &JSXElement) {
                let JSXElementName::Ident(parent) = &node.opening.name else {
                    node.visit_children_with(self);
                    return;
                };
                if !is_component(&parent.sym) {
                    for child in &node.children {
                        let JSXElementChild::JSXElement(child) = child else {
                            continue;
                        };
                        let JSXElementName::Ident(tag) = &child.opening.name else {
                            continue;
                        };
                        if !is_component(&tag.sym) && !is_valid_html_nesting(&parent.sym, &tag.sym)
                        {
                            self.0.errors.push(
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
        }
        program.visit_with(&mut Validator(self));
    }
}
