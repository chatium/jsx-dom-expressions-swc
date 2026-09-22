/// Which renderer to emit for. The universal renderer and per-tag `renderers` are not ported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Generate {
    #[default]
    Dom,
    Ssr,
}

/// The Babel plugin's options, minus the ones that only the universal renderer reads.
/// An empty `effect_wrapper` or `memo_wrapper` stands for upstream's `false`.
#[derive(Debug, Clone)]
pub struct Config {
    pub module_name: String,
    pub generate: Generate,
    pub hydratable: bool,
    pub delegate_events: bool,
    pub delegated_events: Vec<String>,
    pub built_ins: Vec<String>,
    pub wrap_conditionals: bool,
    pub omit_nested_closing_tags: bool,
    pub context_to_custom_elements: bool,
    pub static_marker: String,
    pub effect_wrapper: String,
    pub memo_wrapper: String,
    /// When set, only files whose comments name this library in `@jsxImportSource` are
    /// transformed.
    pub require_import_source: Option<String>,
    pub validate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            module_name: "dom".into(),
            generate: Generate::Dom,
            hydratable: false,
            delegate_events: true,
            delegated_events: Vec::new(),
            built_ins: Vec::new(),
            wrap_conditionals: true,
            omit_nested_closing_tags: false,
            context_to_custom_elements: false,
            static_marker: "@once".into(),
            effect_wrapper: "effect".into(),
            memo_wrapper: "memo".into(),
            require_import_source: None,
            validate: true,
        }
    }
}
