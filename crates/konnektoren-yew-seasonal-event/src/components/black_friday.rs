use crate::{BlackFridayEvent, use_seasonal_event};
use yew::prelude::*;

const GUMROAD_PROFILE_URL: &str = "https://gumroad.konnektoren.help";

/// Text and URL are supplied by the application so they can be translated.
#[derive(Properties, PartialEq)]
pub struct BlackFridayProps {
    pub title: AttrValue,
    pub message: AttrValue,
    pub action_label: AttrValue,
    pub action_url: AttrValue,
    #[prop_or_default]
    pub force_visible: bool,
}

#[function_component(BlackFriday)]
pub fn black_friday(props: &BlackFridayProps) -> Html {
    let visible = use_seasonal_event::<BlackFridayEvent>(props.force_visible);
    if !visible {
        return Html::default();
    }

    html! {
        <section class="black-friday" aria-label={props.title.clone()}>
            <div class="black-friday__intro">
                <div class="black-friday__sparkles" aria-hidden="true">{"✦  ✳  ✦"}</div>
                <h2 class="black-friday__title">{&props.title}</h2>
                <p class="black-friday__message">{&props.message}</p>
                <a class="black-friday__link" href={props.action_url.clone()}>
                    {&props.action_label}<span aria-hidden="true">{" ↗"}</span>
                </a>
                <span class="black-friday__smile" aria-hidden="true">{"✺"}</span>
            </div>
            <div class="black-friday__product">
                <iframe
                    class="black-friday__embed"
                    title={props.action_label.clone()}
                    src={GUMROAD_PROFILE_URL}
                    loading="lazy"
                />
            </div>
        </section>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        BlackFriday,
        BlackFridayProps {
            title: "Black Friday at Konnektoren".into(),
            message: "Take a look at our Gumroad store this Black Friday.".into(),
            action_label: "Visit our Gumroad store".into(),
            action_url: GUMROAD_PROFILE_URL.into(),
            force_visible: true,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn forced_event_renders_the_store_link() {
        let html = yew::ServerRenderer::<BlackFriday>::with_props(|| BlackFridayProps {
            title: "Black Friday".into(),
            message: "Visit the store".into(),
            action_label: "Shop".into(),
            action_url: "https://example.com".into(),
            force_visible: true,
        })
        .render()
        .await;
        assert!(html.contains("Black Friday"));
        assert!(html.contains("https://example.com"));
        assert!(html.contains("https://gumroad.konnektoren.help"));
        assert!(html.contains("<iframe"));
    }
}
