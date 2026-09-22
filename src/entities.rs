/// `html-entities`' `decode`, which the upstream plugin applies to component and fragment
/// text children.
pub(crate) fn decode(text: &str) -> String {
    htmlize::unescape(text).into_owned()
}
