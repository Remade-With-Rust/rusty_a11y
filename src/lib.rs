#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Accessibility helpers for UI chrome -- ARIA HTML string builders.
//!
//! Sibling of [`thoth`](https://github.com/Remade-With-Rust/thoth) (glyphs) and
//! [`rusty_tokens`](https://github.com/Remade-With-Rust/rusty_tokens) (design tokens).
//! Emits small HTML snippets with `role` / `aria-*` only -- no DOM crate, no JS.
//! Requires `alloc` (provided by `std` in normal apps).

pub mod label;
pub mod live;
pub mod status;
