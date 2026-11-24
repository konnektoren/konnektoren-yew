//! The data a [`FilteredList`](crate::FilteredList) shows: filters, choices, sort and
//! translated labels. Plain values, no rendering.
use yew::AttrValue;

/// One choice of a filter or the sort order.
#[derive(Clone, Debug, PartialEq)]
pub struct ListChoice {
    /// Stable form / URL value.
    pub value: AttrValue,
    /// Translated, e.g. "Wins (12)".
    pub label: AttrValue,
}

impl ListChoice {
    pub fn new(value: impl Into<AttrValue>, label: impl Into<AttrValue>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// How a filter is shown.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ListFilterControl {
    #[default]
    Select,
    /// A button with `aria-pressed` for two choices: off is the first, on the second.
    /// `icon` is a Font Awesome name like `fa-star`.
    Toggle { icon: AttrValue },
}

/// A filter control: a select, or a toggle for two choices.
#[derive(Clone, Debug, PartialEq)]
pub struct ListFilter {
    /// Passed back in [`FilteredListProps::on_filter`](crate::FilteredListProps::on_filter).
    pub id: AttrValue,
    /// Translated.
    pub label: AttrValue,
    pub control: ListFilterControl,
    /// The chosen value.
    pub value: AttrValue,
    pub choices: Vec<ListChoice>,
    /// Disabled; a toggle that is on stays enabled so it can be turned off.
    pub disabled: bool,
}

/// The sort select.
#[derive(Clone, Debug, PartialEq)]
pub struct ListSort {
    pub value: AttrValue,
    pub choices: Vec<ListChoice>,
}

/// Translated texts of a [`FilteredList`](crate::FilteredList).
#[derive(Clone, Debug, PartialEq)]
pub struct ListLabels {
    pub search: AttrValue,
    pub search_placeholder: AttrValue,
    /// After the count, e.g. "Attempts".
    pub noun: AttrValue,
    /// Accessible name of the list.
    pub list: AttrValue,
    /// Nothing there at all.
    pub empty: AttrValue,
    /// Nothing matches the search and filters.
    pub no_match: AttrValue,
    pub sort_by: AttrValue,
    /// Above toggles.
    pub show: AttrValue,
    pub clear: AttrValue,
    pub show_more: AttrValue,
}

impl Default for ListLabels {
    fn default() -> Self {
        Self {
            search: "Search".into(),
            search_placeholder: "Name".into(),
            noun: "Entries".into(),
            list: "List".into(),
            empty: "Nothing here yet.".into(),
            no_match: "Nothing matches these filters.".into(),
            sort_by: "Sort by".into(),
            show: "Show".into(),
            clear: "Clear filters".into(),
            show_more: "Show more".into(),
        }
    }
}
