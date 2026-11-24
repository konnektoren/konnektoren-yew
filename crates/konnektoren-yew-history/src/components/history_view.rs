use super::page::{PageState, use_history_page};
use super::{EntryContext, History, HistoryEntry};
use crate::model::{
    FilterContext, FilterControl, HistoryFilters, HistoryItem, HistoryOption, HistoryQuery,
    HistorySort, Search, select,
};
use konnektoren_yew_core::{
    ErrorState, FilteredList, ListChoice, ListFilter, ListFilterControl, ListItem, ListLabels,
    ListSort,
};
use yew::prelude::*;

/// Sets filter `id` to `value`; `true` when the query changed.
fn set_filter<H: History>(state: &mut PageState<H>, id: &str, value: &str) -> bool {
    let mut filters = state.query.filters.clone();
    filters.set(id, value)
        && state.set_query(HistoryQuery {
            filters,
            ..state.query.clone()
        })
}

/// A complete history page for any [`History`]: heading, summary, search, filters,
/// sort, count, list with "Show more", empty states and footer.
///
/// ```ignore
/// html! { <HistoryView<ExerciseHistory> history={history} /> }
/// ```
#[function_component(HistoryView)]
pub fn history_view<H: History>(props: &H::Properties) -> Html {
    let page = use_history_page::<H>(props);
    let state = page.state.borrow();
    let context = page.context(props, &state);
    let labels = H::labels();
    let header = state.history.view_header(&context);
    let root = classes!("history-page", (!H::CLASS.is_empty()).then_some(H::CLASS));
    let items = H::items(props);
    let items = match &items {
        Ok(items) => items,
        Err(message) => {
            return html! {
                <div class={root}>
                    {header}
                    <ErrorState message={AttrValue::from(context.t(message))} />
                </div>
            };
        }
    };
    let filter_context = FilterContext {
        props,
        history: &state.history,
    };
    let name = |item: &H::Item| AttrValue::from(context.t(&item.name()));
    let listed = |filters: &H::Filters, item: &H::Item| {
        filters.admits(item, &filter_context) && state.history.admits(&filter_context, item)
    };
    let sort = state.query.sort;
    let entries = select(
        items,
        &state.query.search,
        name,
        |item| listed(&state.query.filters, item),
        |entry| sort.key(entry),
        sort.direction(),
    );
    // How many records a filter choice would list, with the other choices kept.
    let search = Search::new(&state.query.search);
    let count = |id: &str, value: &str| {
        let mut filters = state.query.filters.clone();
        filters.set(id, value);
        items
            .iter()
            .filter(|item| listed(&filters, item) && search.matches(*item, &name(item)))
            .count()
    };
    let filters = state
        .query
        .filters
        .fields(&filter_context)
        .into_iter()
        .map(|field| ListFilter {
            id: field.id.into(),
            label: context.t(field.label).into(),
            control: match field.control {
                FilterControl::Select => ListFilterControl::Select,
                FilterControl::Toggle { icon } => ListFilterControl::Toggle { icon: icon.into() },
            },
            value: field.value.into(),
            choices: field
                .choices
                .iter()
                .map(|choice| {
                    let label = context.t(choice.label);
                    let label = match field.counts {
                        true => format!("{label} ({})", count(field.id, choice.value)),
                        false => label,
                    };
                    ListChoice::new(choice.value, label)
                })
                .collect(),
            disabled: field.disabled,
        })
        .collect::<Vec<_>>();
    let sort_select = ListSort {
        value: sort.value().into(),
        choices: <H::Sort as HistoryOption>::ALL
            .iter()
            .map(|option| ListChoice::new(option.value(), context.t(option.label())))
            .collect(),
    };
    let total = items.len();
    let matching = entries.len();
    let entries = entries
        .into_iter()
        .take(state.shown)
        .map(|entry| {
            let entry_context = EntryContext {
                index: entry.index,
                name: entry.name,
                history: &state.history,
                base: &context,
            };
            ListItem::new(
                entry.index,
                HistoryEntry::<H>::view(entry.item, &entry_context),
            )
        })
        .collect::<Vec<_>>();
    let list_labels = ListLabels {
        search: context.t(labels.search_label).into(),
        search_placeholder: context.t(labels.search_placeholder).into(),
        noun: context.t(labels.noun).into(),
        list: context.t(labels.title).into(),
        empty: context.t(labels.empty).into(),
        no_match: context.t(labels.no_match).into(),
        sort_by: context.t("Sort by").into(),
        show: context.t("Show").into(),
        clear: context.t("Clear filters").into(),
        show_more: context.t("Show more").into(),
    };

    html! {
        <div class={root}>
            {header}
            {state.history.view_summary(&context, items)}
            <FilteredList
                labels={list_labels}
                search={AttrValue::from(state.query.search.clone())}
                {filters}
                sort={Some(sort_select)}
                controls={state.history.view_controls(&context)}
                items={entries}
                {matching}
                {total}
                active={!search.is_empty() || state.query.filters.active()}
                aside={state.history.view_aside(&context)}
                empty_action={state.history.view_empty_action(&context)}
                on_search={page.callback(|search: String, state: &mut PageState<H>| {
                    let query = HistoryQuery { search, ..state.query.clone() };
                    state.set_query(query)
                })}
                on_filter={page.callback(|(id, value): (AttrValue, AttrValue), state: &mut PageState<H>| {
                    set_filter(state, &id, &value)
                })}
                on_sort={page.callback(|value: String, state: &mut PageState<H>| {
                    let query = HistoryQuery { sort: H::Sort::parse(&value), ..state.query.clone() };
                    state.set_query(query)
                })}
                on_clear={page.callback(|_: (), state: &mut PageState<H>| {
                    let cleared = state.query.cleared();
                    state.set_query(cleared)
                })}
                on_more={page.callback(|_: (), state: &mut PageState<H>| {
                    state.shown += H::PAGE;
                    true
                })}
            />
            {state.history.view_footer(&context)}
        </div>
    }
}
