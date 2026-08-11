#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Accessibility helpers for UI chrome -- ARIA HTML string builders.
//!
//! Sibling of [`rusty_symbols`](https://crates.io/crates/rusty_symbols) and
//! [`rusty_tokens`](https://crates.io/crates/rusty_tokens).
//! Emits small HTML snippets with `role` / `aria-*` only -- no DOM crate, no JS.
//! Requires `alloc` (provided by `std` in normal apps).
//!
//! By default installs [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc)
//! via [`rusty_alloc_default`](https://crates.io/crates/rusty_alloc_default)
//! (opt out with `default-features = false`).

/// Whether this build pulled in the default `rusty_alloc` install.
pub const fn rusty_alloc_enabled() -> bool {
    cfg!(feature = "rusty-alloc")
}

/// Whether the hardened `secure` allocator profile is compiled in.
pub const fn secure_allocator_enabled() -> bool {
    cfg!(feature = "secure")
}

pub mod label;
pub mod live;
pub mod status;
