use super::{ChallengeActions, ChallengeActionsComponent};
use crate::components::ProgressBar;
use crate::prelude::ReadText;
use konnektoren_core::challenges::{ChallengeInput, ChallengeResult, GapFill, GapFillAnswer};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::{ChallengeEvent, Event};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GapFillComponentProps {
    pub challenge: GapFill,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
}

#[function_component(GapFillComponent)]
pub fn gap_fill_component(props: &GapFillComponentProps) -> Html {
    let task_index = use_state(|| 0);
    let challenge_result = use_state(|| ChallengeResult::GapFill(Vec::new()));
    let show_help = use_state(|| false);
    let selected_gap = use_state(|| None::<usize>);
    let current_answers = use_state(Vec::<String>::default);

    if *task_index >= props.challenge.questions.len() {
        return html! {};
    }

    let current_question = &props.challenge.questions[*task_index];

    let handle_gap_click = {
        let selected_gap = selected_gap.clone();
        Callback::from(move |index: usize| {
            selected_gap.set(Some(index));
        })
    };

    let handle_option_selection = {
        let task_index = task_index.clone();
        let challenge_result = challenge_result.clone();
        let current_answers = current_answers.clone();
        let selected_gap = selected_gap.clone();
        let challenge = props.challenge.clone();
        let on_event = props.on_event.clone();
        let on_command = props.on_command.clone();

        Callback::from(move |option: String| {
            let mut answers = (*current_answers).clone();
            if let Some(gap_index) = *selected_gap {
                if answers.len() <= gap_index {
                    answers.resize(gap_index + 1, String::new());
                }
                answers[gap_index] = option;
                current_answers.set(answers.clone());
                selected_gap.set(None);

                // Check if all gaps are filled
                if answers.len() == challenge.questions[*task_index].gaps.len()
                    && !answers.iter().any(|a| a.is_empty())
                {
                    let answer = GapFillAnswer {
                        question_index: *task_index,
                        answers: answers.clone(),
                    };

                    let mut challenge_result_update = (*challenge_result).clone();
                    challenge_result_update
                        .add_input(ChallengeInput::GapFill(answer.clone()))
                        .unwrap();
                    challenge_result.set(challenge_result_update.clone());

                    if let Some(on_event) = on_event.as_ref() {
                        if challenge.check_answer(&answer) {
                            on_event
                                .emit(Event::Challenge(ChallengeEvent::SolvedCorrect(*task_index)));
                        } else {
                            on_event.emit(Event::Challenge(ChallengeEvent::SolvedIncorrect(
                                *task_index,
                            )));
                        }
                    }

                    // Move to next question or finish
                    if *task_index < challenge.questions.len() - 1 {
                        task_index.set(*task_index + 1);
                        current_answers.set(Vec::new());
                        if let Some(on_command) = on_command.as_ref() {
                            on_command.emit(Command::Challenge(ChallengeCommand::NextTask));
                        }
                    } else if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::Finish(Some(
                            challenge_result_update,
                        ))));
                    }
                }
            }
        })
    };

    let handle_action = {
        let task_index = task_index.clone();
        let show_help = show_help.clone();
        let current_answers = current_answers.clone();
        let total_questions = props.challenge.questions.len();
        let on_command = props.on_command.clone();

        Callback::from(move |action: ChallengeActions| match action {
            ChallengeActions::Next => {
                if *task_index < total_questions - 1 {
                    task_index.set(*task_index + 1);
                    current_answers.set(Vec::new());
                    if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::NextTask));
                    }
                }
            }
            ChallengeActions::Previous => {
                if *task_index > 0 {
                    task_index.set(*task_index - 1);
                    current_answers.set(Vec::new());
                    if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::PreviousTask));
                    }
                }
            }
            ChallengeActions::Help => {
                show_help.set(!*show_help);
            }
        })
    };

    html! {
        <div class="gap-fill">
            <ProgressBar
                value={*task_index}
                max={props.challenge.questions.len()}
                label={format!("Question {} of {}", *task_index + 1, props.challenge.questions.len())}
            />

            <div class="gap-fill__sentence">
                {render_sentence(
                    &current_question.sentence,
                    &current_answers,
                    *selected_gap,
                    handle_gap_click.clone(),
                )}
            </div>

            <div class="gap-fill__options">
                {current_question.gaps.iter().flat_map(|gap| &gap.options).cloned().map(|option| {
                    let on_click = {
                        let option = option.clone();
                        let handle_option_selection = handle_option_selection.clone();
                        Callback::from(move |_| handle_option_selection.emit(option.clone()))
                    };

                    html! {
                        <button
                            class="gap-fill__option"
                            onclick={on_click}
                        >
                            {option}
                        </button>
                    }
                }).collect::<Html>()}
            </div>

            if *show_help {
                <div class="gap-fill__hints">
                    <div class="gap-fill__hints-title">{"Hints:"}</div>
                    <ul class="gap-fill__hints-list">
                        {current_question.hints.iter().map(|hint| {
                            html! { <li>{hint}</li> }
                        }).collect::<Html>()}
                    </ul>
                    <div class="gap-fill__translation">
                        {&current_question.translation}
                    </div>
                </div>
            }

            <ChallengeActionsComponent on_action={handle_action} />
            <ReadText text={current_question.explanation.clone()} lang="de-DE" />
        </div>
    }
}

fn render_sentence(
    sentence: &str,
    current_answers: &[String],
    selected_gap: Option<usize>,
    on_gap_click: Callback<usize>,
) -> Html {
    let parts: Vec<&str> = sentence.split("__").collect();

    html! {
        {parts.iter().enumerate().map(|(i, part)| {
            let html = html! { {part} };

            if i < parts.len() - 1 {
                let gap_index = i;
                let gap_value = current_answers.get(gap_index).cloned().unwrap_or_default();
                let is_selected = selected_gap == Some(gap_index);

                let on_click = {
                    let on_gap_click = on_gap_click.clone();
                    let gap_index = gap_index;
                    Callback::from(move |_| on_gap_click.emit(gap_index))
                };

                let gap_class = classes!(
                    "gap-fill__gap",
                    is_selected.then(|| "gap-fill__gap--selected"),
                );

                html! {
                    <>
                        {html}
                        <span class={gap_class} onclick={on_click}>
                            {if gap_value.is_empty() { "___" } else { &gap_value }}
                        </span>
                    </>
                }
            } else {
                html
            }
        }).collect::<Html>()}
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::prelude::{ChallengeType, Game};
    use yew_preview::prelude::*;

    fn create_default_challenge() -> GapFill {
        let game = Game::default();
        let default_challenge = game.create_challenge("past-tense-1").unwrap();
        match &default_challenge.challenge_type {
            ChallengeType::GapFill(gap_fill) => gap_fill.clone(),
            _ => unreachable!(),
        }
    }

    yew_preview::create_preview!(
        GapFillComponent,
        GapFillComponentProps {
            challenge: create_default_challenge(),
            on_command: None,
            on_event: None,
        },
    );
}
