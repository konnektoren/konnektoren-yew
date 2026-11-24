//! Basic building blocks shared by `konnektoren-yew` and its crates (history, FAQ,
//! review, navigation, …).
//!
//! Only **stateless** components for now: they render their props, keep no state and
//! fetch nothing. Texts come in translated (`ErrorState` only falls back to a
//! translated "Try again").
//!
//! - [`model`]: small value types — [`Tone`], [`TimeValue`], formatting helpers.
//! - [`components`]: [`Icon`], [`Pill`], [`StatGrid`], [`Time`] and the status panels
//!   [`EmptyState`], [`ErrorState`], [`LoadingState`], [`StatusMessage`].
//!
//! Styles: `styles/crates/core/core.css` in the `konnektoren-yew` repository, BEM, one
//! file per block.

pub mod components;
pub mod model;

pub use components::*;
pub use model::*;
