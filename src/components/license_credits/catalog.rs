use super::group::CreditGroupView;
use crate::model::{CreditAssetServer, CreditsCatalog};
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
    html! { <>{for props.catalog.groups.iter().map(|group| html! {
        <CreditGroupView key={group.id.clone()} group={group.clone()} server={props.server.clone()} base_url={props.base_url.clone()} />
    })}</> }
}
