use super::sort::{base_order, compare_keys};
use super::{Direction, HistoryItem, SortEntry, SortKey};
use yew::AttrValue;

/// What the player chose: search text, filters and sort order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryQuery<F, S> {
    pub search: String,
    pub filters: F,
    pub sort: S,
}

impl<F: Default, S: Copy> HistoryQuery<F, S> {
    /// No search and default filters; keeps the sort.
    pub fn cleared(&self) -> Self {
        Self {
            search: String::new(),
            filters: F::default(),
            sort: self.sort,
        }
    }
}

/// A record that passed the filters and the search.
pub struct Selected<'a, T> {
    /// Position in the stored history.
    pub index: usize,
    pub item: &'a T,
    /// Translated name, as shown and searched.
    pub name: AttrValue,
}

/// Case-insensitive search in the translated name and the item's search texts.
pub struct Search(String);

impl Search {
    pub fn new(text: &str) -> Self {
        Self(text.trim().to_lowercase())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn matches<T: HistoryItem>(&self, item: &T, name: &str) -> bool {
        self.is_empty()
            || name.to_lowercase().contains(&self.0)
            || item
                .search_texts()
                .iter()
                .any(|text| text.to_lowercase().contains(&self.0))
    }
}

/// The records to list: filtered by `admits`, matched against `search`, newest first,
/// then stably sorted by `key` in `direction`.
pub fn select<'a, T: HistoryItem>(
    items: &'a [T],
    search: &str,
    name: impl Fn(&T) -> AttrValue,
    admits: impl Fn(&T) -> bool,
    key: impl Fn(&SortEntry<'_, T>) -> SortKey,
    direction: Direction,
) -> Vec<Selected<'a, T>> {
    let search = Search::new(search);
    let mut selected: Vec<_> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| admits(item))
        .map(|(index, item)| Selected {
            index,
            item,
            name: name(item),
        })
        .filter(|entry| search.matches(entry.item, &entry.name))
        .collect();
    selected
        .sort_by(|a, b| base_order((a.index, a.item), (b.index, b.item), Direction::Descending));
    let mut keyed: Vec<_> = selected
        .into_iter()
        .map(|entry| {
            let key = key(&SortEntry {
                index: entry.index,
                item: entry.item,
                name: &entry.name,
            });
            (key, entry)
        })
        .collect();
    keyed.sort_by(|(a_key, a), (b_key, b)| match (a_key, b_key) {
        (SortKey::Base, SortKey::Base) => {
            base_order((a.index, a.item), (b.index, b.item), direction)
        }
        _ => compare_keys(a_key, b_key, direction),
    });
    keyed.into_iter().map(|(_, entry)| entry).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};

    #[derive(Clone, PartialEq)]
    struct Item {
        name: &'static str,
        day: Option<u32>,
        score: Option<f64>,
    }

    impl HistoryItem for Item {
        fn key(&self) -> AttrValue {
            self.name.into()
        }
        fn time(&self) -> Option<DateTime<Utc>> {
            self.day
                .and_then(|day| Utc.with_ymd_and_hms(2026, 11, day, 12, 0, 0).single())
        }
        fn search_texts(&self) -> Vec<AttrValue> {
            vec!["extra".into()]
        }
    }

    fn item(name: &'static str, day: Option<u32>, score: Option<f64>) -> Item {
        Item { name, day, score }
    }

    fn order(
        items: &[Item],
        search: &str,
        key: impl Fn(&SortEntry<'_, Item>) -> SortKey,
        direction: Direction,
    ) -> Vec<usize> {
        select(
            items,
            search,
            |item| item.name.into(),
            |_| true,
            key,
            direction,
        )
        .into_iter()
        .map(|entry| entry.index)
        .collect()
    }

    #[test]
    fn base_order_is_newest_first_with_undated_last_by_position() {
        // The same case as konnektoren-yew's challenge history tests.
        let items = [
            item("a", Some(3), None),
            item("b", None, None),
            item("c", Some(1), None),
            item("d", None, None),
            item("e", Some(3), None),
        ];
        assert_eq!(
            order(&items, "", |_| SortKey::Base, Direction::Descending),
            [4, 0, 2, 3, 1]
        );
        // Oldest first keeps undated records last.
        assert_eq!(
            order(&items, "", |_| SortKey::Base, Direction::Ascending),
            [2, 0, 4, 1, 3]
        );
    }

    #[test]
    fn missing_numbers_go_last_and_ties_stay_newest_first() {
        let items = [
            item("a", Some(1), Some(50.0)),
            item("b", Some(2), None),
            item("c", Some(3), Some(90.0)),
            item("d", Some(4), Some(50.0)),
        ];
        let score = |entry: &SortEntry<'_, Item>| SortKey::Number(entry.item.score);
        assert_eq!(
            order(&items, "", score, Direction::Descending),
            [2, 3, 0, 1]
        );
        assert_eq!(order(&items, "", score, Direction::Ascending), [3, 0, 2, 1]);
    }

    #[test]
    fn searches_names_and_search_texts_case_insensitively_and_sorts_text() {
        let items = [
            item("Silent Night", Some(1), None),
            item("little Waltz", Some(3), None),
            item("O Tannenbaum", Some(2), None),
        ];
        let name = |entry: &SortEntry<'_, Item>| SortKey::Text(entry.name.into());
        assert_eq!(
            order(&items, " NIGHT ", |_| SortKey::Base, Direction::Descending),
            [0]
        );
        assert_eq!(order(&items, "t", name, Direction::Ascending), [1, 2, 0]);
        assert_eq!(
            order(&items, "extra", |_| SortKey::Base, Direction::Descending),
            [1, 2, 0]
        );
    }

    #[test]
    fn filters_before_sorting_and_clearing_keeps_the_sort() {
        let items = [
            item("a", Some(1), Some(10.0)),
            item("b", Some(2), Some(90.0)),
        ];
        let strong = select(
            &items,
            "",
            |i| i.name.into(),
            |i| i.score > Some(50.0),
            |_| SortKey::Base,
            Direction::Descending,
        );
        assert_eq!(strong.len(), 1);
        assert_eq!(strong[0].name, "b");
        let query = HistoryQuery {
            search: "x".into(),
            filters: 3,
            sort: 'n',
        };
        assert_eq!(
            query.cleared(),
            HistoryQuery {
                search: String::new(),
                filters: 0,
                sort: 'n'
            }
        );
    }
}
