use super::{OptionsComponent, QuestionComponent};
use crate::components::challenge::multiple_choice::{
    create_handle_option_selection, MultipleChoiceComponentProps,
};
use crate::components::ProgressBar;
use crate::prelude::ReadText;
use konnektoren_core::challenges::ChallengeResult;
use yew::prelude::*;

#[function_component(MultipleChoiceCircleComponent)]
pub fn multiple_choice_circle_component(props: &MultipleChoiceComponentProps) -> Html {
    let task_index = use_state(|| 0);
    let challenge_result = use_state(ChallengeResult::default);
    let show_help = use_state(|| false);

    if *task_index >= props.challenge.questions.len() {
        return html! {};
    }

    let handle_option_selection = create_handle_option_selection(
        task_index.clone(),
        props.challenge.clone(),
        challenge_result.clone(),
        props.challenge.questions.len(),
        props.on_command.clone(),
        props.on_event.clone(),
    );

    html! {
        <div class="multiple-choice-circle">
            <ProgressBar
                value={*task_index}
                max={props.challenge.questions.len()}
                label={format!("Question {} of {}", *task_index + 1, props.challenge.questions.len())}
            />
            <OptionsComponent
                options={props.challenge.options.clone()}
                on_select={handle_option_selection}
            />
            <QuestionComponent
                question={props.challenge.questions[*task_index].clone()}
                help={*show_help}
            />
            <ReadText text={props.challenge.questions[*task_index].question.clone()} lang="de-DE" />
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::prelude::{ChallengeType, Game};
    use yew_preview::prelude::*;

    use konnektoren_core::challenges::multiple_choice::MultipleChoice;

    fn create_default_challenge() -> MultipleChoice {
        let game = Game::default();
        let default_challenge = game.create_challenge("konnektoren-1").unwrap();
        match &default_challenge.challenge_type {
            ChallengeType::MultipleChoice(multiple_choice) => multiple_choice.clone(),
            _ => unreachable!(),
        }
    }

    fn create_articles_challenge() -> MultipleChoice {
        let game = Game::default();
        let default_challenge = game.create_challenge("articles-1").unwrap();
        match &default_challenge.challenge_type {
            ChallengeType::MultipleChoice(multiple_choice) => multiple_choice.clone(),
            _ => unreachable!(),
        }
    }

    yew_preview::create_preview!(
        MultipleChoiceCircleComponent,
        MultipleChoiceComponentProps {
            challenge: create_default_challenge(),
            on_command: None,
            on_event: None,
        },
        (
            "Articles",
            MultipleChoiceComponentProps {
                challenge: create_articles_challenge(),
                on_command: None,
                on_event: None,
            }
        ),
    );
}
