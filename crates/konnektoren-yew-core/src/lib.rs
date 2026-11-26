//! Basic building blocks shared by `konnektoren-yew` and its crates (history, FAQ,
//! review, navigation, …).
//!
//! Visual components render their props and fetch nothing. Texts come in translated (`ErrorState` only falls back to a
//! translated "Try again").
//!
//! - [`model`]: small value types — [`Tone`], [`TimeValue`], formatting helpers.
//! - [`components`]: [`Icon`], [`Pill`], [`StatGrid`], [`Time`] and the status panels
//!   [`EmptyState`], [`ErrorState`], [`LoadingState`], [`StatusMessage`].
//! - [`providers`]: shared context providers such as [`SpeechProvider`].
//! - [`EntryRow`] and [`EntryParts`]: a reusable selectable row with optional facts,
//!   a result, time and actions.
//!
//! Styles: `styles/crates/core/core.css` in the `konnektoren-yew` repository, BEM, one
//! file per block.

pub mod components;
pub mod model;
pub mod providers;

pub use components::*;
pub use model::*;
pub use providers::*;
