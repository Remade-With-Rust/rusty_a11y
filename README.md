# rusty_a11y

[![Remade With Rust](https://img.shields.io/badge/Remade%20With-Rust-000?logo=rust&logoColor=fff)](https://github.com/remade-with-rust)
[![By Mata Network](https://img.shields.io/badge/by-Mata%20Network-5b2be0)](https://www.mata.network)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE-MIT)
![Platforms: Windows · macOS · Linux · Web · WASM](https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux%20%C2%B7%20Web%20%C2%B7%20WASM-informational)
![MSRV: 1.73](https://img.shields.io/badge/MSRV-1.73-informational)

> **rusty_a11y** is an open-source accessibility helper toolkit for Rust UI
> chrome -- **labelled glyphs, ARIA live regions, and status announcements**
> as small HTML string builders -- pure Rust, no DOM crate and no JS. By
> default installs [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc)
> (opt out below). Pair with [`rusty_symbols`](https://crates.io/crates/rusty_symbols)
> and [`rusty_tokens`](https://crates.io/crates/rusty_tokens).

> **Status -- v0.2.0.** Pin
> `rusty_a11y = "0.2"` from crates.io (or git tag `v0.2.0`).
> Requires `alloc` (normal `std` apps already have it). `no_std` + `alloc` ready.

---

## The headline

> **Chrome that screen readers can hear.** Glyphs get `role="img"` names;
> sync/saved/offline updates ride polite or assertive live regions -- without
> pulling a browser DOM binding into your crate graph.

| Dimension | Bare glyph / silent UI | **rusty_a11y** | Goal |
|---|:---:|:---:|:---:|
| Glyph naming | none | **`label::img`** | a11y |
| Status updates | silent | **`live` / `status`** | a11y |
| Escaping | easy to forget | **attribute / text escape** | safe |
| Allocator | system / C | **`rusty_alloc` by default** | opt-out |
| Dependencies | DOM crates | **none when opted out** | maintain |
| License | mixed | **MIT** | -- |

---

## Install

```toml
rusty_a11y = "0.2"
# bring your own allocator:
# rusty_a11y = { version = "0.2", default-features = false }
# hardened:
# rusty_a11y = { version = "0.2", features = ["secure"] }
```

| Feature | Default | Provides |
|---------|---------|----------|
| `rusty-alloc` | **yes** | [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc) via [`rusty_alloc_default`](https://crates.io/crates/rusty_alloc_default) |
| `secure` | no | enables `rusty-alloc` + hardening |

MSRV: **1.73**. Combining with sibling crate defaults is safe (shared allocator seam).

## Quick start

```rust
use rusty_a11y::{label, live, status};

fn verified() -> String {
    // Often paired with rusty_symbols::status::OK in apps
    label::img("\u{2713}", "verified")
}

fn syncing() -> String {
    live::polite("Syncing")
}

fn saved() -> String {
    status::announce(status::Kind::Saved)
}
```

```sh
cargo test
```

## Features

- **label** -- `img(glyph, aria_label)`, `named(visible, aria_label)`.
- **live** -- `polite` / `assertive` live-region HTML.
- **status** -- `Kind::{Saved,Syncing,Offline,Error,Ready}` + `announce` / `announce_error`.
- **Guards** -- doctests + unit tests for escapes and ARIA attrs.

### Capability table

| Capability | Status |
|---|---|
| `label::img` / `named` | done |
| `live::polite` / `assertive` | done |
| `status::announce` | done |
| Quote / HTML escaping | done |
| `no_std` + `alloc` + wasm | done |
| crates.io | done v0.1.0 |

## Architecture

```text
┌──────────────────────────────────────────────────────────┐
│  rusty_a11y                                              │
│                                                          │
│  label   -- img / named                               ✅ │
│  live    -- polite / assertive regions                ✅ │
│  status  -- Kind + announce / announce_error          ✅ │
└──────────────────────────────────────────────────────────┘
```

Northern star: WAI-ARIA patterns for chrome (labelled graphics, live regions,
status). Document-engine / canvas a11y is out of scope.

Sibling toolkits: [thoth](https://github.com/Remade-With-Rust/thoth) ·
[rusty_tokens](https://github.com/Remade-With-Rust/rusty_tokens).

## Platform support

| Platform | Status |
|---|---|
| Windows | yes |
| macOS | yes |
| Linux | yes |
| Web (Dioxus / browsers) | yes |
| WASM (`wasm32-unknown-unknown`) | yes (`alloc`) |

No OS APIs. Inject the returned HTML strings into WebView / Dioxus via
`dangerous_inner_html` or equivalent.

## Remade With Rust

**Remade With Rust** ([Mata Network](https://www.mata.network)) rebuilds essential
tooling in Rust -- memory safety, predictable performance, permissive license.

-> **[github.com/remade-with-rust](https://github.com/remade-with-rust)**

Family: [thoth](https://github.com/Remade-With-Rust/thoth) ·
[rusty_tokens](https://github.com/Remade-With-Rust/rusty_tokens) ·
**rusty_a11y**

## License

MIT -- [LICENSE-MIT](LICENSE-MIT).

## Trademark

"Remade With Rust", "Mata", and "Mata Network" are marks of Mata Network.
