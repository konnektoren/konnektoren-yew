use crate::components::Tour;
use crate::i18n::use_i18n;
use crate::providers::use_settings;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
    pub data: String,
}

#[function_component(TourButton)]
pub fn tour_button(props: &Props) -> Html {
    let i18n = use_i18n();
    let settings = use_settings();
    // Incremented each click to force a fresh remount of Tour (resetting its internal state)
    let tour_key = use_state(|| 0u32);
    // Tracks whether Tour has ever been started this session
    let tour_started = use_state(|| false);

    let on_click = {
        let tour_key = tour_key.clone();
        let tour_started = tour_started.clone();
        Callback::from(move |_| {
            tour_key.set(*tour_key + 1);
            tour_started.set(true);
        })
    };

    if !settings.show_helpers {
        return html! { <></> };
    }

    html! {
        <>
            // Tour is rendered (and remounted via key) whenever tour_started is true.
            // When the user dismisses the tour, yew-tou-rs hides its own overlay internally.
            // The next button click bumps tour_key, forcing a fresh remount that restarts the tour.
            if *tour_started {
                <div class="tour-welcome">
                    <Tour
                        key={*tour_key}
                        id={props.id.clone()}
                        data={props.data.clone()}
                    />
                </div>
            }
            // Button is always visible when show_helpers is true.
            // While the tour overlay is active it sits above the button (z-index 9998 vs 1000),
            // so the button is visually hidden without needing extra state tracking.
            <div class="tour-button">
                <button
                    class="tour-button__btn"
                    onclick={on_click}
                    title={i18n.t("Start Tour")}
                    aria-label={i18n.t("Start Tour")}
                >
                    <span class="tour-button__icon">{"?"}</span>
                    <span class="tour-button__label">{ i18n.t("Start Tour") }</span>
                </button>
            </div>
        </>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        TourButton,
        Props {
            id: "tour".to_string(),
            data: r#"steps:
  - selector: ".tour-button"
    content: "Welcome to Konnektoren! Click here any time to restart the tour."
  - selector: ".select-theme"
    content: "Select a theme"
  - selector: ".select-design"
    content: "Select a design""#
                .to_string()
        },
    );
}
