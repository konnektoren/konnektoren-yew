use chrono::{DateTime, Utc};
use yew::AttrValue;

/// The data of one history record: an exercise attempt, a ranked game, …
///
/// How it is shown is up to [`HistoryEntry`](crate::HistoryEntry) or
/// [`DefaultHistoryEntry`](crate::DefaultHistoryEntry).
pub trait HistoryItem: Clone + PartialEq + 'static {
    /// Identifies what the record is about, e.g. the exercise id. Several records may
    /// share a key; the list itself is keyed by position.
    fn key(&self) -> AttrValue;

    /// When it happened. Records without a time are ordered by their position only
    /// and come after dated ones.
    fn time(&self) -> Option<DateTime<Utc>> {
        None
    }

    /// Untranslated name; translated before it is shown, searched and sorted.
    fn name(&self) -> AttrValue {
        self.key()
    }

    /// More text the search should find, e.g. a piece name. Not translated.
    fn search_texts(&self) -> Vec<AttrValue> {
        Vec::new()
    }
}
