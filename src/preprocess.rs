use swc_core::common::Spanned;
use swc_core::common::comments::Comments;
use swc_core::ecma::ast::*;

use crate::Transform;

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
}
