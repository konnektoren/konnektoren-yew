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
            <button class="challenge-actions__button challenge-actions__button--previous" onclick={on_previous}>
                {"Previous"}
            </button>
            <button class="challenge-actions__button" onclick={on_next}>
                {"Next"}
            </button>
            <button class="challenge-actions__button challenge-actions__button--help" onclick={on_help}>
                {"Help"}
            </button>
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
