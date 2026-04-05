use crate::i18n::use_i18n;
use yew::prelude::*;

pub enum ChallengeActions {
    Next,
    Previous,
    Help,
}

#[derive(Properties, PartialEq)]
pub struct ChallengeActionsComponentProps {
    pub on_action: Callback<ChallengeActions>,
}

#[function_component(ChallengeActionsComponent)]
pub fn challenge_actions_component(props: &ChallengeActionsComponentProps) -> Html {
    let i18n = use_i18n();

    let on_previous = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(ChallengeActions::Previous))
    };

    let on_next = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(ChallengeActions::Next))
    };

    let on_help = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(ChallengeActions::Help))
    };

    html! {
        <div class="challenge-actions">
            <div class="challenge-actions__buttons">
                <button class="challenge-actions__button challenge-actions__button--previous" onclick={on_previous}>
                    { i18n.t("Previous") }
                </button>
                <button class="challenge-actions__button challenge-actions__button--next" onclick={on_next}>
                    { i18n.t("Next") }
                </button>
                <button class="challenge-actions__button challenge-actions__button--help" onclick={on_help}>
                    { i18n.t("Help") }
                </button>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeActionsComponent,
        ChallengeActionsComponentProps {
            on_action: Callback::noop()
        },
    );
}
