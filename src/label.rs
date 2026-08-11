//! Labelled graphics and control naming helpers.

extern crate alloc;

use alloc::string::String;

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

/// Wrap a glyph in a labelled image role for screen readers.
///
/// ```
/// use rusty_a11y::label;
/// let s = label::img("\u{2713}", "verified");
/// assert!(s.contains("aria-label=\"verified\""));
/// assert!(s.contains("role=\"img\""));
/// ```
pub fn img(glyph: &str, aria_label: &str) -> String {
    let safe = escape_attr(aria_label);
    alloc::format!(r#"<span role="img" aria-label="{safe}">{glyph}</span>"#)
}

/// Associate visible text with an explicit accessible name via `aria-label`.
pub fn named(visible: &str, aria_label: &str) -> String {
    let safe = escape_attr(aria_label);
    let vis = escape_attr(visible);
    alloc::format!(r#"<span aria-label="{safe}">{vis}</span>"#)
}
