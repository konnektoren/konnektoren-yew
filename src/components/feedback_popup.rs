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

    // Sync state with props when they change
    {
        let show = show.clone();
        let expanded = expanded.clone();
        use_effect_with(
            (props.show, props.expanded),
            move |(prop_show, prop_expanded)| {
                show.set(*prop_show);
                expanded.set(*prop_expanded);
            },
        );
    }

    // Handle timeout
    {
        let show = show.clone();
        let timeout = props.timeout_seconds;
        use_effect_with(timeout, move |&timeout| {
            let handle = if timeout > 0 {
                Some(Timeout::new(timeout * 1000, move || {
                    show.set(true);
                }))
            } else {
                None
            };

            move || drop(handle)
        });
    }

    let on_close = {
        let expanded = expanded.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            expanded.set(false);
        })
    };

    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            expanded.set(!*expanded);
        })
    };

    if !*show {
        return html! {};
    }

    let container_class = classes!("feedback", (*expanded).then_some("feedback--expanded"));

    html! {
        <div class={container_class}>
            <button
                class="feedback__bubble"
                onclick={on_toggle}
                type="button"
                aria-label={i18n.t("Open feedback form")}
                aria-expanded={expanded.to_string()}
            >
                <span class="feedback__bubble-icon" aria-hidden="true">{"💬"}</span>
                <span class="feedback__bubble-text">{i18n.t("Feedback")}</span>
            </button>

            <div class="feedback__content" role="dialog" aria-labelledby="feedback-title">
                <button
                    class="feedback__close"
                    onclick={on_close}
                    type="button"
                    aria-label={i18n.t("Close feedback form")}
                >
                    {"×"}
                </button>
                <h3 id="feedback-title" class="feedback__title">
                    {i18n.t("We'd love your feedback!")}
                </h3>
                <ChallengeReviewComponent
                    api_url={props.api_url.clone()}
                    challenge_id="general_feedback"
                />
            </div>
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
            "Expanded",
            FeedbackPopupProps {
                api_url: "https://api.konnektoren.help".to_string(),
                timeout_seconds: 0,
                show: true,
                expanded: true,
            }
        ),
        (
            "With Timeout",
            FeedbackPopupProps {
                api_url: "https://api.konnektoren.help".to_string(),
                timeout_seconds: 5,
                show: true,
                expanded: false,
            }
        ),
        (
            "Hidden Initially",
            FeedbackPopupProps {
                api_url: "https://api.konnektoren.help".to_string(),
                timeout_seconds: 10,
                show: false,
                expanded: false,
            }
        )
    );
}
