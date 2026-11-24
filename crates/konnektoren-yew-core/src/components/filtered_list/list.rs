use super::{ListCount, ListFilterField, ListSearch, ListSortField};
use crate::components::EmptyState;
use crate::model::{ListFilter, ListLabels, ListSort};
use yew::prelude::*;
use yew::virtual_dom::Key;

/// One rendered entry; keyed so rows keep their state while the list changes.
#[derive(Clone, PartialEq)]
pub struct ListItem {
    pub key: Key,
    pub content: Html,
}

impl ListItem {
    pub fn new(key: impl Into<Key>, content: Html) -> Self {
        Self {
            key: key.into(),
            content,
        }
    }
}

/// A reusable list of keyed, caller-rendered entries.
#[derive(Properties, PartialEq)]
pub struct ListRowsProps {
    pub items: Vec<ListItem>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub item_class: Classes,
    #[prop_or_default]
    pub label: Option<AttrValue>,
}

#[function_component(ListRows)]
pub fn list_rows(props: &ListRowsProps) -> Html {
    html! {
        <ol class={props.class.clone()} aria-label={props.label.clone()}>
            {for props.items.iter().map(|item| html! {
                <li key={item.key.clone()} class={props.item_class.clone()}>{item.content.clone()}</li>
            })}
        </ol>
    }
}

#[derive(Properties, PartialEq)]
pub struct FilteredListProps {
    #[prop_or_default]
    pub labels: ListLabels,
    /// The search text.
    #[prop_or_default]
    pub search: AttrValue,
    #[prop_or_default]
    pub filters: Vec<ListFilter>,
    /// `None` or one choice: no sort select.
    #[prop_or_default]
    pub sort: Option<ListSort>,
    /// Override the search field for specialized search interactions.
    #[prop_or_default]
    pub search_control: Option<Html>,
    /// Content between the search controls and the result count.
    #[prop_or_default]
    pub before_results: Html,
    /// More form controls after the filters.
    #[prop_or_default]
    pub controls: Html,
    /// The visible entries, already searched, filtered, sorted and paged.
    pub items: Vec<ListItem>,
    /// Entries matching search and filters (`items` may show fewer).
    pub matching: usize,
    /// All entries.
    pub total: usize,
    /// Search or a filter is set: shows "Clear filters".
    #[prop_or_default]
    pub active: bool,
    /// A panel beside the list, e.g. details of the selected entry.
    #[prop_or_default]
    pub aside: Option<Html>,
    /// Next to the empty message, e.g. a link to start.
    #[prop_or_default]
    pub empty_action: Html,
    #[prop_or_default]
    pub on_search: Callback<String>,
    /// `(filter id, choice value)`.
    #[prop_or_default]
    pub on_filter: Callback<(AttrValue, AttrValue)>,
    #[prop_or_default]
    pub on_sort: Callback<String>,
    #[prop_or_default]
    pub on_clear: Callback<()>,
    /// "Show more"; shown while `items` has fewer than `matching`.
    #[prop_or_default]
    pub on_more: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
}

/// Search, filters and sort above a paged list with a count, "Clear filters", empty
/// states and an optional side panel. Stateless; see the module docs.
#[function_component(FilteredList)]
pub fn filtered_list(props: &FilteredListProps) -> Html {
    let labels = &props.labels;
    let visible = props.items.len();
    let aside = props.aside.clone();

    html! {
        <div class={classes!("filtered-list", props.class.clone())}>
            <form class="filtered-list__filters" role="search"
                onsubmit={Callback::from(|event: SubmitEvent| event.prevent_default())}>
                if let Some(search_control) = &props.search_control {
                    {search_control.clone()}
                } else {
                    <ListSearch label={labels.search.clone()} placeholder={labels.search_placeholder.clone()}
                        value={props.search.clone()} on_input={props.on_search.clone()} />
                }
                {for props.filters.iter().map(|filter| html! {
                    <ListFilterField key={filter.id.to_string()} filter={filter.clone()}
                        show={labels.show.clone()} on_change={props.on_filter.clone()} />
                })}
                {props.controls.clone()}
                if let Some(sort) = &props.sort {
                    <ListSortField sort={sort.clone()} label={labels.sort_by.clone()} on_change={props.on_sort.clone()} />
                }
            </form>
            {props.before_results.clone()}
            <ListCount matching={props.matching} total={props.total} noun={labels.noun.clone()}
                active={props.active} clear_label={labels.clear.clone()} on_clear={props.on_clear.clone()} />
            <div class={classes!("filtered-list__layout", aside.is_some().then_some("filtered-list__layout--open"))}>
                if let Some(aside) = aside {
                    <aside class="filtered-list__aside">{aside}</aside>
                }
                <div class="filtered-list__results">
                    if props.total == 0 {
                        <EmptyState message={labels.empty.clone()}>
                            {props.empty_action.clone()}
                        </EmptyState>
                    } else if props.matching == 0 {
                        <EmptyState message={labels.no_match.clone()}>
                            <button type="button" class="filtered-list__reset"
                                onclick={props.on_clear.reform(|_: MouseEvent| ())}>{labels.clear.clone()}</button>
                        </EmptyState>
                    } else {
                        <ListRows class="filtered-list__list" item_class="filtered-list__item"
                            label={Some(labels.list.clone())} items={props.items.clone()} />
                        if visible < props.matching {
                            <button type="button" class="filtered-list__more"
                                onclick={props.on_more.reform(|_: MouseEvent| ())}>
                                {labels.show_more.clone()}
                                <span class="filtered-list__remaining">{format!("{visible} / {}", props.matching)}</span>
                            </button>
                        }
                    }
                </div>
            </div>
        </div>
    }
}
