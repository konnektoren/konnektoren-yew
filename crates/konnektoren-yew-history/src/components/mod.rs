//! The Yew side of a history: the [`History`] page trait, the entry layer, and the
//! components rendering them.

#[cfg(feature = "challenge")]
pub mod challenge;
mod entry;
mod entry_row;
mod history;
mod history_preview;
mod history_view;
mod page;

pub use entry::{
    DefaultHistoryEntry, EntryContext, EntryMeta, EntryParts, EntryResult, HistoryEntry,
};
pub use entry_row::{HistoryEntryRow, HistoryEntryRowProps};
pub use history::{History, HistoryContext};
pub use history_preview::{HistoryPreview, HistoryPreviewProps};
pub use history_view::HistoryView;
