use super::group::CreditGroupView;
use crate::i18n::use_i18n;
use crate::model::{CreditAssetServer, CreditsCatalog};
use konnektoren_yew_core::{FilteredList, ListItem, ListLabels};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct CreditsViewProps {
    pub(super) catalog: Arc<CreditsCatalog>,
    pub(super) server: CreditAssetServer,
    pub(super) base_url: AttrValue,
}

#[function_component(CreditsView)]
pub(super) fn credits_view(props: &CreditsViewProps) -> Html {
    let i18n = use_i18n();
    let search = use_state(String::new);
    let query = search.trim().to_lowercase();
    let active = !query.is_empty();
    let items: Vec<ListItem> = props.catalog.groups.iter().filter_map(|group| {
        let mut group = group.clone();
        if active && !i18n.t(&group.name).to_lowercase().contains(&query) {
            group.credits.retain(|credit| {
                i18n.t(&credit.name).to_lowercase().contains(&query)
                    || credit.description.iter().any(|text| i18n.t(text).to_lowercase().contains(&query))
                    || credit.licenses.iter().any(|license| i18n.t(&license.name).to_lowercase().contains(&query))
            });
            if group.credits.is_empty() { return None; }
        }
        Some(ListItem::new(group.id.clone(), html! {
            <CreditGroupView {group} server={props.server.clone()} base_url={props.base_url.clone()} expanded={active} />
        }))
    }).collect();
    let matching = items.len();
    let on_search = {
        let search = search.clone();
        Callback::from(move |value| search.set(value))
    };
    let on_clear = {
        let search = search.clone();
        Callback::from(move |()| search.set(String::new()))
    };
    let labels = ListLabels {
        search: i18n.t("Search").into(),
        search_placeholder: i18n.t("Name").into(),
        noun: i18n.t("Credit groups").into(),
        list: i18n.t("Credits and licenses").into(),
        empty: i18n.t("Nothing here yet.").into(),
        no_match: i18n.t("Nothing matches these filters.").into(),
        sort_by: i18n.t("Sort by").into(),
        show: i18n.t("Show").into(),
        clear: i18n.t("Clear filters").into(),
        show_more: i18n.t("Show more").into(),
    };
    html! {
        <FilteredList {labels} {items} {matching} total={props.catalog.groups.len()}
            search={(*search).clone()} {active} {on_search} {on_clear} />
    }
}
