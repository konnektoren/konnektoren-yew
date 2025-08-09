use crate::i18n::use_i18n;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GumroadSubscribeProps {
    pub subscribe_url: String,
}

#[function_component(GumroadSubscribeComponent)]
pub fn gumroad_subscribe_component(props: &GumroadSubscribeProps) -> Html {
    let i18n = use_i18n();

    html! {
        <div class="gumroad-subscribe-ad">
            <div class="gumroad-subscribe-ad__message">
                { i18n.t("Want more interactive exercises?") }
            </div>
            <div class="gumroad-subscribe-ad__cta">
                { i18n.t("Subscribe to stay up to date and unlock exclusive new content!") }
            </div>
            <a
                class="gumroad-subscribe-ad__button"
                href={props.subscribe_url.clone()}
                target="_blank"
                rel="noopener"
            >
                { i18n.t("Subscribe now") }
            </a>
            <div class="gumroad-subscribe-ad__note">
                { i18n.t("No spam. Unsubscribe anytime.") }
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        GumroadSubscribeComponent,
        GumroadSubscribeProps {
            subscribe_url: "https://konnektoren.gumroad.com/subscribe".to_string(),
        },
    );
}
