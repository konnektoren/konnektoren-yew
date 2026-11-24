use super::{HistoryItem, HistoryOption};
use crate::components::History;
use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use yew::AttrValue;

/// What a sort order compares. The crate applies the rules: stable sorting on top of
/// newest first, and missing values always last.
#[derive(Clone, Debug, PartialEq)]
pub enum SortKey {
    /// The base order: by time, then by position in the history.
    Base,
    Time(Option<DateTime<Utc>>),
    Number(Option<f64>),
    /// Compared case-insensitively.
    Text(AttrValue),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    #[default]
    Descending,
}

impl Direction {
    fn apply(self, ordering: Ordering) -> Ordering {
        match self {
            Self::Ascending => ordering,
            Self::Descending => ordering.reverse(),
        }
    }
}

/// One record while sorting.
pub struct SortEntry<'a, T> {
    /// Position in the stored history.
    pub index: usize,
    pub item: &'a T,
    /// Translated name, as shown.
    pub name: &'a str,
}

/// A sort order. Say *by what* ([`key`](Self::key)) and in which
/// [`direction`](Self::direction); ties keep the newest first.
pub trait HistorySort<H: History>: HistoryOption {
    fn key(self, entry: &SortEntry<'_, H::Item>) -> SortKey;

    fn direction(self) -> Direction {
        Direction::Descending
    }
}

/// Newest or oldest first: for histories without other orders.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimeSort {
    #[default]
    Newest,
    Oldest,
}

impl HistoryOption for TimeSort {
    const ALL: &'static [Self] = &[Self::Newest, Self::Oldest];

    fn value(self) -> &'static str {
        match self {
            Self::Newest => "newest",
            Self::Oldest => "oldest",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Newest => "Newest first",
            Self::Oldest => "Oldest first",
        }
    }
}

impl<H: History> HistorySort<H> for TimeSort {
    fn key(self, _: &SortEntry<'_, H::Item>) -> SortKey {
        SortKey::Base
    }

    fn direction(self) -> Direction {
        match self {
            Self::Newest => Direction::Descending,
            Self::Oldest => Direction::Ascending,
        }
    }
}

/// Present values first in either direction, then by `compare` in `direction`.
fn present_first<T>(
    a: Option<T>,
    b: Option<T>,
    direction: Direction,
    compare: impl Fn(&T, &T) -> Ordering,
) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => direction.apply(compare(&a, &b)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

/// The base order between two records in `direction`: dated before undated, then by
/// time, then by position.
pub(crate) fn base_order<T: HistoryItem>(
    (a_index, a): (usize, &T),
    (b_index, b): (usize, &T),
    direction: Direction,
) -> Ordering {
    present_first(a.time(), b.time(), direction, Ord::cmp)
        .then_with(|| direction.apply(a_index.cmp(&b_index)))
}

/// Compares two keys of the same kind; different kinds and `Base` are equal here.
pub(crate) fn compare_keys(a: &SortKey, b: &SortKey, direction: Direction) -> Ordering {
    match (a, b) {
        (SortKey::Time(a), SortKey::Time(b)) => present_first(*a, *b, direction, Ord::cmp),
        (SortKey::Number(a), SortKey::Number(b)) => {
            present_first(*a, *b, direction, |a, b| a.total_cmp(b))
        }
        (SortKey::Text(a), SortKey::Text(b)) => {
            direction.apply(a.to_lowercase().cmp(&b.to_lowercase()))
        }
        _ => Ordering::Equal,
    }
}
