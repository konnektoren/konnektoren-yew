//! Searchable, filterable and sortable history pages for Konnektoren applications.
//!
//! Re-exported by `konnektoren-yew` as `konnektoren_yew::history` with its `history`
//! feature (on by default).
//!
//! Split into [`model`] (the domain: records, options, filters, sort orders, the
//! query, labels, tones and times — no rendering) and [`components`] (the `History`
//! page trait, the entry layer and the Yew components). Everything is also exported
//! at the crate root.
//!
//! The page API is written like Yew struct components (apps implement [`History`]
//! with `create` / `update` / `view_*`); the crate renders it with function
//! components. Two layers:
//! - **History:** a page type implements [`History`] (records, filters, sort orders,
//!   props, messages and optional parts around the list). [`HistoryView`] renders the
//!   whole page and [`HistoryPreview`] the newest records.
//! - **Entry:** each record implements [`HistoryItem`] (key, time, name) and either
//!   [`DefaultHistoryEntry`] (what the standard row shows) or [`HistoryEntry`] (its own
//!   row).
//!
//! Filters are [`HistoryFilter`] enums combined in a tuple; sort orders are
//! [`HistorySort`] enums returning a [`SortKey`]. Ties stay newest first and missing
//! values sort last.
//!
//! With the `challenge` feature, [`Challenge`](konnektoren_core::challenges::Challenge)
//! is a record, with the [`challenge::Results`] filter and
//! [`challenge::ChallengeSort`] orders.
//!
//! Styles: `styles/crates/history/history.css` in the `konnektoren-yew` repository.

pub mod components;
pub mod model;

#[cfg(any(test, feature = "yew-preview"))]
pub mod sample;

/// Challenge attempts as history records (feature `challenge`): the domain in
/// [`model::challenge`], the result badge in [`components::challenge`].
#[cfg(feature = "challenge")]
pub mod challenge {
    pub use crate::components::challenge::*;
    pub use crate::model::challenge::*;
}

pub use components::{
    DefaultHistoryEntry, EntryContext, EntryMeta, EntryParts, EntryResult, History, HistoryContext,
    HistoryEntry, HistoryEntryRow, HistoryEntryRowProps, HistoryPreview, HistoryPreviewProps,
    HistoryView,
};
pub use model::{
    Direction, FilterChoice, FilterContext, FilterControl, FilterField, HistoryChange,
    HistoryFilter, HistoryFilters, HistoryItem, HistoryLabels, HistoryOption, HistoryQuery,
    HistorySort, Search, Selected, SortEntry, SortKey, TimeSort, TimeValue, Tone, select,
};
