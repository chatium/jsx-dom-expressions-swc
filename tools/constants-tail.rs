pub(crate) static RESERVED_NAMESPACES: &[&str] =
    &["class", "on", "oncapture", "style", "use", "prop", "attr"];

pub(crate) static NON_SPREAD_NAMESPACES: &[&str] = &["class", "style", "use", "prop", "attr"];

/// `PropAliases` from dom-expressions: an alias plus the tag names it applies to.
/// An empty tag list means the alias applies to every tag.
static PROP_ALIASES: &[(&str, &str, &[&str])] = &[
    ("class", "className", &[]),
    ("formnovalidate", "formNoValidate", &["BUTTON", "INPUT"]),
    ("ismap", "isMap", &["IMG"]),
    ("nomodule", "noModule", &["SCRIPT"]),
    ("playsinline", "playsInline", &["VIDEO"]),
    ("readonly", "readOnly", &["INPUT", "TEXTAREA"]),
];

pub(crate) fn get_prop_alias(prop: &str, tag_name: &str) -> Option<&'static str> {
    let (_, alias, tags) = PROP_ALIASES.iter().find(|(p, _, _)| *p == prop)?;
    if tags.is_empty() || tags.contains(&tag_name) {
        Some(alias)
    } else {
        None
    }
}
