//! [![github]](https://github.com/tamaskis/plotting)&ensp;[![crates-io]](https://crates.io/crates/plotting)&ensp;[![docs-rs]](https://docs.rs/plotting)&ensp;[![mdbook]](https://tamaskis.github.io/plotting/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//! [mdbook]: https://img.shields.io/badge/mdbook-ffd92f?style=for-the-badge&labelColor=555555&logo=bookstack
//!
//! Plotting.

// Linter setup.
#![warn(missing_docs)]

// Module declarations.
mod colors;
mod figure;
mod format;
mod line_style;
mod quick;
mod trace;

// Re-exports.
pub use crate::colors::{Color, NamedColor};
pub use crate::figure::Figure;
pub use crate::format::Format;
pub use crate::format::FormatBuilder;
pub use crate::line_style::LineStyle;
pub use crate::quick::{
    quick_plot_2d, quick_plot_2d_with_labels, quick_plot_3d, quick_plot_3d_with_labels,
};
pub use crate::trace::Trace;
