use super::page::use_history_page;
use super::{EntryContext, History, HistoryEntry};
use crate::model::{Direction, HistoryItem, SortKey, select};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryPreviewProps<P: Properties> {
    /// The props the full [`HistoryView`](super::HistoryView) takes.
    pub data: P,
    /// Newest records to show.
    #[prop_or(5)]
    pub limit: usize,
    /// Beside the title, e.g. a link to the full history.
    #[prop_or_default]
    pub more: Html,
    /// Instead of the history's title.
    #[prop_or_default]
    pub title: Option<AttrValue>,
}

/// The newest records of a [`History`] in compact rows, e.g. on a profile.
#[function_component(HistoryPreview)]
pub fn history_preview<H: History>(props: &HistoryPreviewProps<H::Properties>) -> Html {
    let page = use_history_page::<H>(&props.data);
    let state = page.state.borrow();
    let context = page.context(&props.data, &state);
    let items = H::items(&props.data);
    let labels = H::labels();
    let title = props
        .title
        .clone()
        .unwrap_or_else(|| context.t(labels.title).into());
    let body = match &items {
        Err(message) => {
            html! { <p class="history-preview__error" role="alert">{context.t(message)}</p> }
        }
        Ok(items) if items.is_empty() || props.limit == 0 => {
            html! { <p class="history-preview__empty">{context.t(labels.empty)}</p> }
        }
        Ok(items) => {
            let entries = select(
                items,
                "",
                |item| context.t(&item.name()).into(),
                |_| true,
                |_| SortKey::Base,
                Direction::Descending,
            );
            html! {
                <ol class="history-preview__list" aria-label={title.clone()}>
                    {for entries.into_iter().take(props.limit).map(|entry| {
                        let entry_context = EntryContext {
                            index: entry.index,
                            name: entry.name,
                            history: &state.history,
                            base: &context,
                        };
                        html! {
                            <li key={entry.index} class="history-preview__item">
                                {HistoryEntry::<H>::view_compact(entry.item, &entry_context)}
                            </li>
                        }
                    })}
                </ol>
            }
        }
    };
    let has_items = matches!(&items, Ok(items) if !items.is_empty());
    html! {
        <section class="history-preview">
            <div class="history-preview__heading">
                <h3 class="history-preview__title">{title}</h3>
                if has_items { {props.more.clone()} }
            </div>
            {body}
        </section>
    }
}
