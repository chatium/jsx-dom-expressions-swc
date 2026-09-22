use std::cell::RefCell;
use std::rc::Rc;

use swc_core::common::Span;

/// A diagnostic the Babel plugin raises by throwing, which aborts its transform. This pass
/// records it instead and carries on, so the caller decides what to do with partial output.
#[derive(Debug, Clone)]
pub struct CompileError {
    pub span: Span,
    pub message: String,
}

/// A shared, cheaply cloned handle the pass writes diagnostics into.
#[derive(Clone, Default)]
pub struct Errors(Rc<RefCell<Vec<CompileError>>>);

impl Errors {
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    /// Drains the collected diagnostics.
    pub fn take(&self) -> Vec<CompileError> {
        std::mem::take(&mut *self.0.borrow_mut())
    }

    pub(crate) fn push(&self, span: Span, message: String) {
        self.0.borrow_mut().push(CompileError { span, message });
    }
}

impl std::fmt::Debug for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}
