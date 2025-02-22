use crate::components::ChallengeReviewComponent;
use crate::i18n::use_i18n;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FeedbackPopupProps {
    pub api_url: String,
    #[prop_or(0)]
    pub timeout_seconds: u32,
    #[prop_or(false)]
    pub show: bool,
    #[prop_or(false)]
    pub expanded: bool,
}

#[function_component(FeedbackPopup)]
pub fn feedback_popup(props: &FeedbackPopupProps) -> Html {
    let show = use_state(|| props.show);
    let expanded = use_state(|| props.expanded);
    let i18n = use_i18n();

    {
        let show = show.clone();
        let timeout = props.timeout_seconds;
        use_effect_with((), move |_| {
            let handle = Timeout::new(timeout * 1000, move || {
                show.set(true);
            });

            move || {
                handle.cancel();
            }
        });
    }

    let on_close = {
        let show = show.clone();
        Callback::from(move |_| show.set(false))
    };

    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    if !*show {
        return html! {};
    }

    html! {
        <div class={classes!("feedback", if *expanded { "feedback--expanded" } else { "" })}>
            <button class="feedback__bubble" onclick={on_toggle.clone()}>
                <span class="feedback__bubble-icon">{"💬"}</span>
                <span class="feedback__bubble-text">{i18n.t("Feedback")}</span>
            </button>
            if *expanded {
                <div class="feedback__content">
                    <button class="feedback__close" onclick={on_close}>{"×"}</button>
                    <h3 class="feedback__title">{i18n.t("We'd love your feedback!")}</h3>
                    <ChallengeReviewComponent
                        api_url={props.api_url.clone()}
                        challenge_id="general_feedback"
                    />
                </div>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        FeedbackPopup,
        FeedbackPopupProps {
            api_url: "https://api.konnektoren.help".to_string(),
            timeout_seconds: 0,
            show: true,
            expanded: false,
        },
        (
            "10 Seconds timeout",
            FeedbackPopupProps {
                api_url: "https://api.konnektoren.help".to_string(),
                timeout_seconds: 10,
                show: true,
                expanded: false,
            }
        ),
        (
            "5 Seconds timeout",
            FeedbackPopupProps {
                api_url: "https://api.konnektoren.help".to_string(),
                timeout_seconds: 5,
                show: true,
                expanded: false,
            }
        )
    );
}
