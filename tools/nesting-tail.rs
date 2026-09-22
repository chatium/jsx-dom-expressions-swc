fn lookup<'a>(table: &'a [(&str, &'a [&str])], key: &str) -> Option<&'a [&'a str]> {
    table
        .binary_search_by(|(name, _)| (*name).cmp(key))
        .ok()
        .map(|index| table[index].1)
}

/// `isValidHTMLNesting(parent, child)`.
pub(crate) fn is_valid_html_nesting(parent: &str, child: &str) -> bool {
    if let Some(valid) = lookup(ONLY_VALID_CHILDREN, parent) {
        return valid.contains(&child);
    }
    if let Some(valid) = lookup(ONLY_VALID_PARENTS, child) {
        return valid.contains(&parent);
    }
    if let Some(invalid) = lookup(KNOWN_INVALID_CHILDREN, parent) {
        if invalid.contains(&child) {
            return false;
        }
    }
    if let Some(invalid) = lookup(KNOWN_INVALID_PARENTS, child) {
        if invalid.contains(&parent) {
            return false;
        }
    }
    true
}
