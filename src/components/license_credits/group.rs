use super::license_file::LicenseFileView;
use crate::i18n::use_i18n;
use crate::model::{CreditAssetServer, CreditGroup};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct CreditGroupProps {
    pub(super) group: CreditGroup,
    pub(super) server: CreditAssetServer,
    pub(super) base_url: AttrValue,
}

#[function_component(CreditGroupView)]
pub(super) fn credit_group_view(props: &CreditGroupProps) -> Html {
    let i18n = use_i18n();
    let requested = use_state(|| false);
    let on_open = {
        let requested = requested.clone();
        Callback::from(move |_| requested.set(true))
    };
    html! {
        <details class="license-credits__group">
            <summary onclick={on_open}>{i18n.t(&props.group.name)}</summary>
            if *requested {
                <div class="license-credits__content">
                    {for props.group.credits.iter().map(|credit| html! {
                        <article key={credit.id.clone()} class="license-credits__credit">
                            <h3>{i18n.t(&credit.name)}</h3>
                            {for credit.description.iter().map(|text| html! { <p>{i18n.t(text)}</p> })}
                            <div class="license-credits__links">
                                {for credit.links.iter().map(|link| html! { <a href={link.url.clone()}>{i18n.t(&link.name)}</a> })}
                            </div>
                            {for credit.licenses.iter().map(|license| html! {
                                <LicenseFileView key={license.file.clone()} license={license.clone()} server={props.server.clone()} base_url={props.base_url.clone()} />
                            })}
                        </article>
                    })}
                    {for props.group.notes.iter().map(|text| html! { <p>{i18n.t(text)}</p> })}
                </div>
            }
        </details>
    }
}
