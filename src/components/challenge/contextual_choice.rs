use super::{ChallengeActions, ChallengeActionsComponent};
use crate::components::ProgressBar;
use konnektoren_core::challenges::{ChallengeResult, ContextItemChoiceAnswers, ContextualChoice};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::{ChallengeEvent, Event};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ContextualChoiceComponentProps {
    pub challenge: ContextualChoice,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
}

#[function_component(ContextualChoiceComponent)]
pub fn contextual_choice_component(props: &ContextualChoiceComponentProps) -> Html {
    let item_index = use_state(|| 0);
    let challenge_result = use_state(ChallengeResult::default);
    let show_help = use_state(|| false);
    let selections = use_state(|| HashMap::new());

    if *item_index >= props.challenge.items.len() {
        return html! {};
    }

    let handle_action = create_action_handler(
        item_index.clone(),
        show_help.clone(),
        props.challenge.items.len(),
        props.on_command.clone(),
        selections.clone(),
    );
    let handle_option_selection = create_option_selection_handler(
        item_index.clone(),
        props.challenge.clone(),
        challenge_result.clone(),
        props.on_command.clone(),
        props.on_event.clone(),
        selections.clone(),
    );

    let current_item = &props.challenge.items[*item_index];
    let template_parts: Vec<&str> = current_item.template.split("{}").collect();

    html! {
        <div class="contextual-choice">
            <div class="contextual-choice__progress">
                <ProgressBar
                    value={*item_index}
                    max={props.challenge.items.len()}
                    label={format!("Item {} of {}", *item_index + 1, props.challenge.items.len())}
                />
            </div>
            <div class="contextual-choice__item">
                { render_template_parts(template_parts, current_item, &selections, *item_index, handle_option_selection) }
            </div>
            <ChallengeActionsComponent on_action={handle_action} />
        </div>
    }
}

fn create_action_handler(
    item_index: UseStateHandle<usize>,
    show_help: UseStateHandle<bool>,
    total_items: usize,
    on_command: Option<Callback<Command>>,
    selections: UseStateHandle<HashMap<(usize, usize), usize>>,
) -> Callback<ChallengeActions> {
    Callback::from(move |action: ChallengeActions| match action {
        ChallengeActions::Next => {
            handle_next_action(&item_index, total_items, &on_command, &selections)
        }
        ChallengeActions::Previous => handle_previous_action(&item_index, &on_command, &selections),
        ChallengeActions::Help => show_help.set(!*show_help),
    })
}

fn handle_next_action(
    item_index: &UseStateHandle<usize>,
    total_items: usize,
    on_command: &Option<Callback<Command>>,
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
) {
    if **item_index < total_items - 1 {
        let next_item_index = **item_index + 1;
        item_index.set(next_item_index);

        let mut new_selections = (**selections).clone();
        new_selections.retain(|&(item, _), _| item != next_item_index);
        selections.set(new_selections);

        if let Some(on_command) = on_command {
            let command = Command::Challenge(ChallengeCommand::NextTask);
            on_command.emit(command);
        }
    }
}

fn handle_previous_action(
    item_index: &UseStateHandle<usize>,
    on_command: &Option<Callback<Command>>,
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
) {
    if **item_index > 0 {
        let previous_item_index = **item_index - 1;
        item_index.set(previous_item_index);

        // Clear selections for the previous item
        let mut new_selections = (**selections).clone();
        new_selections.retain(|&(item, _), _| item != previous_item_index);
        selections.set(new_selections);

        if let Some(on_command) = on_command {
            let command = Command::Challenge(ChallengeCommand::PreviousTask);
            on_command.emit(command);
        }
    }
}

fn create_option_selection_handler(
    item_index: UseStateHandle<usize>,
    challenge: ContextualChoice,
    challenge_result: UseStateHandle<ChallengeResult>,
    on_command: Option<Callback<Command>>,
    on_event: Option<Callback<Event>>,
    selections: UseStateHandle<HashMap<(usize, usize), usize>>,
) -> Callback<(usize, usize)> {
    Callback::from(move |(choice_index, option_index): (usize, usize)| {
        update_selections(&selections, *item_index, choice_index, option_index);
        update_challenge_result(&challenge_result, &selections, *item_index, &challenge);
        handle_event(
            &on_event,
            &challenge,
            *item_index,
            choice_index,
            option_index,
        );
        check_for_task_completion(&item_index, &challenge, &challenge_result, &on_command);
    })
}

fn update_selections(
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
    item_index: usize,
    choice_index: usize,
    option_index: usize,
) {
    let mut new_selections = (**selections).clone();
    new_selections.insert((item_index, choice_index), option_index + 1);
    selections.set(new_selections);
}

fn update_challenge_result(
    challenge_result: &UseStateHandle<ChallengeResult>,
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
    item_index: usize,
    challenge: &ContextualChoice,
) {
    let mut new_result = (**challenge_result).clone();
    match &mut new_result {
        ChallengeResult::ContextualChoice(answers) => {
            if answers.len() <= item_index {
                answers.resize(item_index + 1, ContextItemChoiceAnswers { ids: vec![] });
            }
            let item_choices = &challenge.items[item_index].choices;
            answers[item_index].ids = item_choices
                .iter()
                .enumerate()
                .map(|(choice_index, _)| {
                    (**selections)
                        .get(&(item_index, choice_index))
                        .cloned()
                        .unwrap_or(0)
                })
                .collect();
        }
        _ => {
            new_result =
                ChallengeResult::ContextualChoice(vec![ContextItemChoiceAnswers { ids: vec![] }]);
        }
    }
    challenge_result.set(new_result);
}

fn handle_event(
    on_event: &Option<Callback<Event>>,
    challenge: &ContextualChoice,
    item_index: usize,
    choice_index: usize,
    option_index: usize,
) {
    if let Some(on_event) = on_event {
        if item_index < challenge.items.len()
            && choice_index < challenge.items[item_index].choices.len()
        {
            let is_correct = challenge.items[item_index].choices[choice_index].correct_answer
                == challenge.items[item_index].choices[choice_index].options[option_index];
            if is_correct {
                on_event.emit(Event::Challenge(ChallengeEvent::SolvedCorrect(item_index)));
            } else {
                on_event.emit(Event::Challenge(ChallengeEvent::SolvedIncorrect(
                    item_index,
                )));
            }
        }
    }
}

fn check_for_task_completion(
    item_index: &UseStateHandle<usize>,
    challenge: &ContextualChoice,
    challenge_result: &UseStateHandle<ChallengeResult>,
    on_command: &Option<Callback<Command>>,
) {
    if **item_index < challenge.items.len() {
        if **item_index == challenge.items.len() - 1 {
            let result = (**challenge_result).clone();
            if let Some(on_command) = on_command {
                let command = Command::Challenge(ChallengeCommand::Finish(Some(result)));
                on_command.emit(command);
            }
        } else {
            let next_index = **item_index + 1;
            item_index.set(next_index);
            if let Some(on_command) = on_command {
                let command = Command::Challenge(ChallengeCommand::NextTask);
                on_command.emit(command);
            }
        }
    }
}

fn render_template_parts(
    template_parts: Vec<&str>,
    current_item: &konnektoren_core::challenges::ContextItem,
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
    item_index: usize,
    handle_option_selection: Callback<(usize, usize)>,
) -> Html {
    template_parts
        .iter()
        .enumerate()
        .map(|(i, part)| {
            html! {
                <>
                    <span>{ part }</span>
                    {
                        if i < current_item.choices.len() {
                            render_select(i, current_item, selections, item_index, handle_option_selection.clone())
                        } else {
                            html! {}
                        }
                    }
                </>
            }
        })
        .collect::<Html>()
}

fn render_select(
    choice_index: usize,
    current_item: &konnektoren_core::challenges::ContextItem,
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
    item_index: usize,
    handle_option_selection: Callback<(usize, usize)>,
) -> Html {
    let selected_value = (**selections).get(&(item_index, choice_index)).cloned();

    html! {
        <select
            class="contextual-choice__select"
            value={selected_value.map(|v| v.to_string()).unwrap_or_default()}
            onchange={
                let handle_option_selection = handle_option_selection.clone();
                Callback::from(move |e: web_sys::Event| {
                    let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
                    handle_option_selection.emit((choice_index, target.selected_index() as usize - 1));
                })
            }
        >
            <option value="" disabled=true selected={selected_value.is_none()}>{"Select an option"}</option>
            {
                current_item.choices[choice_index].options.iter().enumerate().map(|(j, option)| {
                    html! { <option value={(j + 1).to_string()} selected={selected_value == Some(j + 1)}>{ option }</option> }
                }).collect::<Html>()
            }
        </select>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{Choice, ContextItem, ContextualChoice};
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ContextualChoiceComponent,
        ContextualChoiceComponentProps {
            challenge: ContextualChoice {
                items: vec![
                    ContextItem {
                        template: "What is the capital of {}?".to_string(),
                        choices: vec![Choice {
                            id: 0,
                            correct_answer: "Berlin".to_string(),
                            options: vec![
                                "Berlin".to_string(),
                                "Munich".to_string(),
                                "Hamburg".to_string(),
                                "Frankfurt".to_string()
                            ]
                        }]
                    },
                    ContextItem {
                        template: "What is the capital of {}?".to_string(),
                        choices: vec![Choice {
                            id: 1,
                            correct_answer: "Paris".to_string(),
                            options: vec![
                                "Paris".to_string(),
                                "Marseille".to_string(),
                                "Lyon".to_string(),
                                "Toulouse".to_string()
                            ]
                        }]
                    }
                ],
                ..Default::default()
            },
            on_command: None,
            on_event: None,
        },
    );
}
