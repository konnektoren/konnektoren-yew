use super::{FilteredList, FilteredListProps, ListItem};
use crate::model::{ListChoice, ListFilter, ListFilterControl, ListLabels, ListSort};
use yew::prelude::*;
use yew_preview::prelude::*;

const WORDS: [&str; 8] = [
    "der Apfel",
    "die Birne",
    "das Brot",
    "der Käse",
    "die Milch",
    "das Ei",
    "der Saft",
    "die Butter",
];

/// A word list as a caller would build it: here already filtered to "der" words.
fn props(shown: usize, filtered: bool) -> FilteredListProps {
    let words: Vec<&str> = WORDS
        .iter()
        .copied()
        .filter(|word| !filtered || word.starts_with("der"))
        .collect();
    FilteredListProps {
        labels: ListLabels {
            search: "Search words".into(),
            search_placeholder: "Word".into(),
            noun: "Words".into(),
            list: "Vocabulary".into(),
            ..ListLabels::default()
        },
        search: AttrValue::default(),
        filters: vec![
            ListFilter {
                id: "article".into(),
                label: "Article".into(),
                control: ListFilterControl::Select,
                value: if filtered { "der" } else { "all" }.into(),
                choices: vec![
                    ListChoice::new("all", "All articles"),
                    ListChoice::new("der", "der (3)"),
                    ListChoice::new("die", "die (3)"),
                    ListChoice::new("das", "das (2)"),
                ],
                disabled: false,
            },
            ListFilter {
                id: "learned".into(),
                label: "Learned".into(),
                control: ListFilterControl::Toggle {
                    icon: "fa-star".into(),
                },
                value: "all".into(),
                choices: vec![
                    ListChoice::new("all", "All"),
                    ListChoice::new("only", "Learned"),
                ],
                disabled: false,
            },
        ],
        sort: Some(ListSort {
            value: "az".into(),
            choices: vec![ListChoice::new("az", "A–Z"), ListChoice::new("za", "Z–A")],
        }),
        controls: Html::default(),
        search_control: None,
        before_results: Html::default(),
        items: words
            .iter()
            .take(shown)
            .map(|word| ListItem::new(*word, html! { <span>{*word}</span> }))
            .collect(),
        matching: words.len(),
        total: WORDS.len(),
        active: filtered,
        aside: None,
        empty_action: Html::default(),
        on_search: Callback::noop(),
        on_filter: Callback::noop(),
        on_sort: Callback::noop(),
        on_clear: Callback::noop(),
        on_more: Callback::noop(),
        class: Classes::new(),
    }
}

yew_preview::create_preview!(
    FilteredList,
    props(5, false),
    ("Filtered", props(5, true)),
    (
        "Nothing matches",
        FilteredListProps {
            items: Vec::new(),
            matching: 0,
            active: true,
            ..props(5, false)
        }
    ),
    (
        "Empty",
        FilteredListProps {
            items: Vec::new(),
            matching: 0,
            total: 0,
            ..props(5, false)
        }
    ),
    (
        "With panel",
        FilteredListProps {
            aside: Some(html! { <p>{"der Apfel — the apple"}</p> }),
            ..props(8, false)
        }
    ),
);
