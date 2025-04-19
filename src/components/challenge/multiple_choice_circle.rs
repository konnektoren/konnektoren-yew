use super::{OptionsComponent, QuestionComponent};
use crate::components::ProgressBar;
use crate::components::challenge::multiple_choice::{
    MultipleChoiceComponentProps, create_handle_option_selection,
};
#[cfg(feature = "effects")]
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

    let read_text = {
        #[cfg(feature = "effects")]
        {
            html! {
                <ReadText
                    text={props.challenge.questions[*task_index].question.clone()}
                    lang={props.challenge.lang.clone()}
                />
            }
        }
        #[cfg(not(feature = "effects"))]
        {
            html! {}
        }
    };

    html! {
        <div class="multiple-choice-circle">
            <ProgressBar
                value={*task_index}
                max={props.challenge.questions.len()}
                label={format!("Question {} of {}", *task_index + 1, props.challenge.questions.len())}
            />
            <div class="multiple-choice-circle__content">
                <OptionsComponent
                    options={props.challenge.options.clone()}
                    on_select={handle_option_selection}
                />
                <QuestionComponent
                    question={props.challenge.questions[*task_index].clone()}
                    help={*show_help}
                />
            </div>
            {read_text}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::prelude::{ChallengeType, Game, MultipleChoiceOption, Question};
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

    fn create_spanish_challenge() -> MultipleChoice {
        let id = "spanish-1".to_string();
        let name = "Spanish Basic Verbs".to_string();
        let lang = "es".to_string();
        let options = vec![
            MultipleChoiceOption {
                id: 1,
                name: "ser".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "estar".to_string(),
            },
            MultipleChoiceOption {
                id: 3,
                name: "tener".to_string(),
            },
            MultipleChoiceOption {
                id: 4,
                name: "hacer".to_string(),
            },
        ];
        let questions = vec![
            Question {
                question: "¿Cuál verbo se usa para expresar características permanentes?"
                    .to_string(),
                help: "Se usa para características esenciales y permanentes".to_string(),
                option: 1, // ser
                image: None,
            },
            Question {
                question: "¿Qué verbo se usa para estados temporales o ubicación?".to_string(),
                help: "Se usa para estados temporales y localización".to_string(),
                option: 2, // estar
                image: None,
            },
            Question {
                question: "¿Cuál es el verbo que significa 'to have'?".to_string(),
                help: "Se usa para expresar posesión".to_string(),
                option: 3, // tener
                image: None,
            },
            Question {
                question: "¿Qué verbo significa 'to do' o 'to make'?".to_string(),
                help: "Se usa para acciones y creación".to_string(),
                option: 4, // hacer
                image: None,
            },
        ];
        MultipleChoice {
            id,
            name,
            lang,
            options,
            questions,
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
        (
            "Spanish",
            MultipleChoiceComponentProps {
                challenge: create_spanish_challenge(),
                on_command: None,
                on_event: None,
            }
        )
    );
}
