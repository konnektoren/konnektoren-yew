use crate::{AdventEvent, use_seasonal_event};
use yew::prelude::*;

/// Applications supply translated text and the destination for their calendar.
#[derive(Properties, PartialEq)]
pub struct AdventProps {
    pub title: AttrValue,
    pub message: AttrValue,
    pub action_label: AttrValue,
    pub action_url: AttrValue,
    #[prop_or_default]
    pub force_visible: bool,
}

#[function_component(Advent)]
pub fn advent(props: &AdventProps) -> Html {
    let visible = use_seasonal_event::<AdventEvent>(props.force_visible);
    if !visible {
        return Html::default();
    }

    html! {
        <section class="advent-event" aria-label={props.title.clone()}>
            <div class="advent-event__decoration" aria-hidden="true">
                <span class="advent-event__star">{"✦"}</span>
                <span class="advent-event__tree">{"🎄"}</span>
                <span class="advent-event__snow">{"❄ ✨ ❄"}</span>
            </div>
            <div class="advent-event__content">
                <h2 class="advent-event__title">{&props.title}</h2>
                <p class="advent-event__message">{&props.message}</p>
                <a class="advent-event__link" href={props.action_url.clone()}>
                    {&props.action_label}<span aria-hidden="true">{" →"}</span>
                </a>
            </div>
        </section>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        Advent,
        AdventProps {
            title: "Advent season is here!".into(),
            message: "Discover a little surprise behind each door this December.".into(),
            action_label: "Explore the Advent calendar".into(),
            action_url: "/advent-calendar".into(),
            force_visible: true,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn forced_event_renders_the_calendar_link() {
        let html = yew::ServerRenderer::<Advent>::with_props(|| AdventProps {
            title: "Advent season".into(),
            message: "A surprise each day".into(),
            action_label: "Explore".into(),
            action_url: "/advent-calendar".into(),
            force_visible: true,
        })
        .render()
        .await;
        assert!(html.contains("Advent season"));
        assert!(html.contains("/advent-calendar"));
    }
}
