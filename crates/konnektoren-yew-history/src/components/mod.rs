//! The Yew side of a history: the [`History`] page trait, the entry layer, and the
//! components rendering them.

#[cfg(feature = "challenge")]
pub mod challenge;
mod entry;
mod history;
mod history_preview;
mod history_view;
mod page;

pub use entry::{DefaultHistoryEntry, EntryContext, HistoryEntry};
pub use history::{History, HistoryContext};
pub use history_preview::{HistoryPreview, HistoryPreviewProps};
pub use history_view::HistoryView;
pub use konnektoren_yew_core::{
    EntryMeta, EntryParts, EntryResult, EntryRow as HistoryEntryRow,
    EntryRowProps as HistoryEntryRowProps,
};
