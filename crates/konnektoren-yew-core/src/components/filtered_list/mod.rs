//! A searchable, filterable, sortable and paged list — **controlled**: the caller owns
//! the query and the paging and passes translated texts, the current values, the
//! visible items and callbacks. `konnektoren-yew-history`'s `HistoryView` is built on
//! it; anything else with a list to narrow down (exercises, FAQ, achievements) can use
//! it the same way.
//!
//! [`FilteredList`] puts the parts together; [`ListSearch`], [`ListFilterField`],
//! [`ListSortField`] and [`ListCount`] also work on their own. The data types
//! ([`ListFilter`](crate::model::ListFilter), [`ListSort`](crate::model::ListSort),
//! [`ListLabels`](crate::model::ListLabels), …) are in [`model`](crate::model).
mod count;
mod filter;
mod list;
#[cfg(feature = "yew-preview")]
mod preview;
mod search;
mod sort;

pub use count::{ListCount, ListCountProps};
pub use filter::{ListFilterField, ListFilterFieldProps};
pub use list::{FilteredList, FilteredListProps, ListItem, ListRows, ListRowsProps};
pub use search::{ListSearch, ListSearchProps};
pub use sort::{ListSortField, ListSortFieldProps};
