use std::collections::HashSet;

use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

/// Babel's `Scope#generateUid`, including its off-by-one suffix quirk: the sequence for
/// `el$` is `_el$, _el$2 … _el$9, _el$0, _el$1, _el$10, _el$11 …`. Matching it exactly is
/// what keeps our output byte-identical to the Babel plugin's.
#[derive(Default)]
pub(crate) struct UidGen {
    taken: HashSet<Atom>,
}

impl UidGen {
    pub(crate) fn from_program(program: &Program) -> Self {
        let mut collector = Collector {
            taken: HashSet::new(),
        };
        program.visit_with(&mut collector);
        Self {
            taken: collector.taken,
        }
    }

    pub(crate) fn generate(&mut self, name: &str) -> Ident {
        // Babel: toIdentifier(name).replace(/^_+/, "").replace(/\d+$/g, "")
        let stem = name
            .trim_start_matches('_')
            .trim_end_matches(|c: char| c.is_ascii_digit());
        let mut i = 0usize;
        loop {
            let mut uid = String::with_capacity(stem.len() + 3);
            uid.push('_');
            uid.push_str(stem);
            if i >= 11 {
                uid.push_str(&(i - 1).to_string());
            } else if i >= 9 {
                uid.push_str(&(i - 9).to_string());
            } else if i >= 1 {
                uid.push_str(&(i + 1).to_string());
            }
            i += 1;
            let uid: Atom = uid.into();
            if self.taken.insert(uid.clone()) {
                return Ident::new_no_ctxt(uid, Default::default());
            }
        }
    }
}

/// Collects every name Babel would consider taken: bindings, references, globals and labels.
/// Property positions are already excluded: SWC types them as `IdentName`, which `visit_ident`
/// never sees. Import/export specifiers are the exception — their remote name is a real `Ident`
/// that Babel does not count as a reference.
struct Collector {
    taken: HashSet<Atom>,
}

impl Visit for Collector {
    fn visit_ident(&mut self, ident: &Ident) {
        self.taken.insert(ident.sym.clone());
    }

    fn visit_import_named_specifier(&mut self, node: &ImportNamedSpecifier) {
        // `imported` is the remote name; only the local binding counts.
        node.local.visit_with(self);
    }

    fn visit_export_named_specifier(&mut self, node: &ExportNamedSpecifier) {
        node.orig.visit_with(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reproduces_babels_uid_sequence() {
        // Checked against @babel/traverse: the suffix jumps back to 0 and 1 after 9.
        let mut uid = UidGen::default();
        let generated: Vec<String> = (0..14)
            .map(|_| uid.generate("el$").sym.to_string())
            .collect();
        assert_eq!(
            generated,
            [
                "_el$", "_el$2", "_el$3", "_el$4", "_el$5", "_el$6", "_el$7", "_el$8", "_el$9",
                "_el$0", "_el$1", "_el$10", "_el$11", "_el$12"
            ]
        );
    }

    #[test]
    fn strips_leading_underscores_and_trailing_digits_like_babel() {
        let mut uid = UidGen::default();
        assert_eq!(uid.generate("_c$").sym.as_str(), "_c$");
        assert_eq!(uid.generate("_$template").sym.as_str(), "_$template");
        assert_eq!(uid.generate("tmpl$2").sym.as_str(), "_tmpl$");
    }

    #[test]
    fn skips_names_already_in_the_program() {
        let mut uid = UidGen {
            taken: ["_el$".into(), "_el$2".into()].into_iter().collect(),
        };
        assert_eq!(uid.generate("el$").sym.as_str(), "_el$3");
    }
}
