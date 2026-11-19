use crate::i18n::use_i18n;
use crate::model::{CreditAssetServer, LicenseTextFormat};
use crate::providers::credits_provider::use_credit_asset;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct LicenseTextProps {
    pub(super) server: CreditAssetServer,
    pub(super) path: AttrValue,
}

#[function_component(LicenseText)]
pub(super) fn license_text(props: &LicenseTextProps) -> Html {
    let i18n = use_i18n();
    let (text, retry) =
        use_credit_asset::<LicenseTextFormat>(props.server.clone(), props.path.clone());
    match text {
        None => html! { <p role="status">{i18n.t("Loading license…")}</p> },
        Some(Err(_)) => {
            html! { <div role="alert"><p>{i18n.t("Could not load license.")}</p><button class="btn btn-outline btn-sm" onclick={retry}>{i18n.t("Retry")}</button></div> }
        }
        Some(Ok(text)) => html! { <pre class="license-credits__text">{text.as_str()}</pre> },
    }
}
