use super::license_text::LicenseText;
use crate::i18n::use_i18n;
use crate::model::{CreditAssetServer, LicenseFile};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct LicenseFileProps {
    pub(super) license: LicenseFile,
    pub(super) server: CreditAssetServer,
    pub(super) base_url: AttrValue,
}

#[function_component(LicenseFileView)]
pub(super) fn license_file_view(props: &LicenseFileProps) -> Html {
    let i18n = use_i18n();
    let requested = use_state(|| false);
    let on_open = {
        let requested = requested.clone();
        Callback::from(move |_| requested.set(true))
    };
    let url = format!(
        "{}/{}",
        props.base_url.trim_end_matches('/'),
        props.license.file.trim_start_matches('/')
    );
    html! {
        <details class="license-credits__license">
            <summary onclick={on_open}>{i18n.t(&props.license.name)}</summary>
            if *requested {
                <LicenseText server={props.server.clone()} path={props.license.file.clone()} />
                <a class="license-credits__file-link" href={url}>{i18n.t("Open license file")}</a>
            }
        </details>
    }
}
