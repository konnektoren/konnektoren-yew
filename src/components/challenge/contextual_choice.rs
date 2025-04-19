use super::{ChallengeActions, ChallengeActionsComponent};
use crate::components::ProgressBar;
use konnektoren_core::challenges::{ChallengeResult, ContextualChoice};
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
    let selections = use_state(HashMap::new);

    // Create a struct to hold the dependencies for our effect
    #[derive(PartialEq)]
    struct EffectDeps {
        selections: HashMap<(usize, usize), usize>,
        item_index: usize,
    }

    // Effect to validate selections when they change
    {
        let item_index = item_index.clone();
        let challenge = props.challenge.clone();
        let challenge_result = challenge_result.clone();
        let on_command = props.on_command.clone();
        let on_event = props.on_event.clone();
        let selections = selections.clone();

        let deps = EffectDeps {
            selections: (*selections).clone(),
            item_index: *item_index,
        };

        use_effect_with(deps, move |deps| {
            if deps.item_index >= challenge.items.len() {
                // Skip processing if index is out of bounds
            } else {
                let current_item = &challenge.items[deps.item_index];

                // Check if all options are selected for this item
                let all_options_selected = current_item
                    .choices
                    .iter()
                    .enumerate()
                    .all(|(idx, _)| deps.selections.get(&(deps.item_index, idx)).is_some());

                if all_options_selected {
                    // Only evaluate when all options are selected using 0-based indices
                    let all_correct =
                        current_item
                            .choices
                            .iter()
                            .enumerate()
                            .all(|(choice_idx, choice)| {
                                if let Some(&selected_idx) =
                                    deps.selections.get(&(deps.item_index, choice_idx))
                                {
                                    // Use 0-based indexing directly
                                    selected_idx < choice.options.len()
                                        && choice.options[selected_idx] == choice.correct_answer
                                } else {
                                    false
                                }
                            });

                    // Emit appropriate event based on correctness
                    if let Some(on_event) = &on_event {
                        if all_correct {
                            on_event.emit(Event::Challenge(ChallengeEvent::SolvedCorrect(
                                deps.item_index,
                            )));
                        } else {
                            on_event.emit(Event::Challenge(ChallengeEvent::SolvedIncorrect(
                                deps.item_index,
                            )));
                        }
                    }

                    // Auto-advance regardless of correctness
                    if deps.item_index < challenge.items.len() - 1 {
                        // Move to next item
                        let next_idx = deps.item_index + 1;

                        // Clear selections for next item
                        let mut new_selections = deps.selections.clone();
                        new_selections.retain(|&(item, _), _| item != next_idx);
                        selections.set(new_selections);

                        // Update item index
                        item_index.set(next_idx);

                        // Emit command
                        if let Some(cmd) = &on_command {
                            cmd.emit(Command::Challenge(ChallengeCommand::NextTask));
                        }
                    } else if all_correct {
                        // Only finish if this is the last item AND all answers are correct
                        if let Some(cmd) = &on_command {
                            cmd.emit(Command::Challenge(ChallengeCommand::Finish(Some(
                                (*challenge_result).clone(),
                            ))));
                        }
                    }
                }
            }

            // Single closure return point
            || {}
        });
    }

    // Keep the option selection handler simple - just update selections and challenge result
    let handle_option_selection = {
        let selections = selections.clone();
        let challenge = props.challenge.clone();
        let challenge_result = challenge_result.clone();
        let item_index = item_index.clone();

        Callback::from(move |(choice_index, option_index): (usize, usize)| {
            #[cfg(feature = "csr")]
            {
                use konnektoren_core::challenges::ContextItemChoiceAnswers;
                // Update selections - STORE 0-BASED INDICES DIRECTLY
                let mut new_selections = (*selections).clone();
                new_selections.insert((*item_index, choice_index), option_index); // No more +1

                // Generate the IDs vector before moving new_selections
                let item_choices = &challenge.items[*item_index].choices;
                let ids: Vec<usize> = item_choices
                    .iter()
                    .enumerate()
                    .map(|(choice_index, _)| {
                        new_selections
                            .get(&(*item_index, choice_index))
                            .cloned()
                            .unwrap_or(0) // Default value for unselected (0 means unselected)
                    })
                    .collect();

                // Now it's safe to move new_selections
                selections.set(new_selections);

                // Update challenge result with the previously calculated IDs
                let mut new_result = (*challenge_result).clone();
                match &mut new_result {
                    ChallengeResult::ContextualChoice(answers) => {
                        if answers.len() <= *item_index {
                            answers
                                .resize(*item_index + 1, ContextItemChoiceAnswers { ids: vec![] });
                        }
                        answers[*item_index].ids = ids;
                    }
                    _ => {
                        new_result =
                            ChallengeResult::ContextualChoice(vec![ContextItemChoiceAnswers {
                                ids: vec![],
                            }]);
                    }
                }
                challenge_result.set(new_result);
            }
        })
    };

    if *item_index >= props.challenge.items.len() {
        return html! {};
    }

    // Force UI update when item_index changes
    let component_key = format!("contextual-choice-item-{}", *item_index);

    let handle_action = create_action_handler(
        item_index.clone(),
        show_help.clone(),
        props.challenge.items.len(),
        props.on_command.clone(),
        selections.clone(),
    );

    let current_item = &props.challenge.items[*item_index];

    html! {
        <div class="contextual-choice" key={component_key}>
            <div class="contextual-choice__progress">
                <ProgressBar
                    value={*item_index}
                    max={props.challenge.items.len()}
                    label={format!("Item {} of {}", *item_index + 1, props.challenge.items.len())}
                />
            </div>
            <div class="contextual-choice__item">
                { render_template_parts(current_item, &selections, *item_index, handle_option_selection) }
            </div>

            // Help section
            if *show_help {
                <div class="contextual-choice__help">
                    <h3 class="contextual-choice__help-title">{"Help"}</h3>
                    <p class="contextual-choice__help-text">
                        {"Fill in the blanks by selecting the appropriate option for each dropdown."}
                    </p>
                    <div class="contextual-choice__help-hints">
                        <h4 class="contextual-choice__help-hints-title">{"Hints:"}</h4>
                        <ul>
                            {
                                current_item.choices.iter().map(|choice| {
                                    html! {
                                        <li class="contextual-choice__help-hint">
                                            {"The correct answer is: "}<strong>{&choice.correct_answer}</strong>
                                        </li>
                                    }
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                </div>
            }

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

        // Clear selections for the next item explicitly
        let mut new_selections = (**selections).clone();
        new_selections.retain(|&(item, _), _| item != next_item_index);
        selections.set(new_selections);

        // Set the new item index after clearing selections
        item_index.set(next_item_index);

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

        // Clear selections for the previous item explicitly
        let mut new_selections = (**selections).clone();
        new_selections.retain(|&(item, _), _| item != previous_item_index);
        selections.set(new_selections);

        // Set the new item index after clearing selections
        item_index.set(previous_item_index);

        if let Some(on_command) = on_command {
            let command = Command::Challenge(ChallengeCommand::PreviousTask);
            on_command.emit(command);
        }
    }
}

fn render_template_parts(
    current_item: &konnektoren_core::challenges::ContextItem,
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
    item_index: usize,
    handle_option_selection: Callback<(usize, usize)>,
) -> Html {
    // Regex to match both {}, {0}, {1}, etc.
    let re = regex::Regex::new(r"\{(\d*)\}").unwrap();

    // First pass: identify which choice indices are explicitly referenced
    let mut used_indices = std::collections::HashSet::new();
    for cap in re.captures_iter(&current_item.template) {
        if let Some(digit_match) = cap.get(1) {
            let digit_str = digit_match.as_str();
            if !digit_str.is_empty() {
                if let Ok(idx) = digit_str.parse::<usize>() {
                    used_indices.insert(idx);
                }
            }
        }
    }

    // Create a list of available indices for unnumbered placeholders
    let mut available_indices: Vec<usize> = (0..current_item.choices.len())
        .filter(|idx| !used_indices.contains(idx))
        .collect();

    // Second pass: generate the HTML
    let mut last_end = 0;
    let mut result = Vec::new();

    for cap in re.captures_iter(&current_item.template) {
        let whole_match = cap.get(0).unwrap();

        // Add the text before this placeholder
        if whole_match.start() > last_end {
            result.push(html! {
                <span>{ &current_item.template[last_end..whole_match.start()] }</span>
            });
        }

        // Determine the choice index to use
        let choice_index = if let Some(digit_match) = cap.get(1) {
            let digit_str = digit_match.as_str();
            if digit_str.is_empty() {
                // For {} (unnumbered), use the next available index
                available_indices.pop().unwrap_or(0)
            } else {
                // For {0}, {1}, etc., use the specified index
                digit_str.parse().unwrap_or(0)
            }
        } else {
            // This shouldn't happen with our regex, but just in case
            0
        };

        // Add the select component for this placeholder
        if choice_index < current_item.choices.len() {
            result.push(render_select(
                choice_index,
                current_item,
                selections,
                item_index,
                handle_option_selection.clone(),
            ));
        }

        last_end = whole_match.end();
    }

    // Add any remaining text after the last placeholder
    if last_end < current_item.template.len() {
        result.push(html! {
            <span>{ &current_item.template[last_end..] }</span>
        });
    }

    html! { <>{for result}</> }
}

fn render_select(
    choice_index: usize,
    current_item: &konnektoren_core::challenges::ContextItem,
    selections: &UseStateHandle<HashMap<(usize, usize), usize>>,
    item_index: usize,
    handle_option_selection: Callback<(usize, usize)>,
) -> Html {
    let selected_value = (**selections).get(&(item_index, choice_index)).cloned();

    // Create a unique ID for this select element based on item_index and choice_index
    let select_id = format!("choice-select-{}-{}", item_index, choice_index);

    #[cfg(feature = "csr")]
    let onchange = {
        let handle_option_selection = handle_option_selection.clone();

        Callback::from(move |e: yew::Event| {
            use wasm_bindgen::JsCast;
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                // Convert selected_index to 0-based index by subtracting 1
                if target.selected_index() > 0 {
                    handle_option_selection
                        .emit((choice_index, (target.selected_index() - 1) as usize));
                }
            }
        })
    };

    #[cfg(not(feature = "csr"))]
    let onchange = Callback::from(|_: yew::Event| ());

    html! {
        <select
            id={select_id.clone()}
            class="contextual-choice__select"
            key={select_id.clone()} // Using clone to avoid move issues
            onchange={onchange}
        >
            <option value="" disabled=true selected={selected_value.is_none()}>
                {"Select an option"}
            </option>
            {
                current_item.choices[choice_index].options.iter().enumerate().map(|(j, option)| {
                    html! {
                        <option
                            value={j.to_string()} // 0-based index
                            selected={selected_value == Some(j)} // Compare with 0-based index
                        >
                            { option }
                        </option>
                    }
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
                id: "default".to_string(),
                name: "Default Example".to_string(),
                description: "Default example with numbered placeholders".to_string(),
                items: vec![
                    ContextItem {
                        template: "What is the capital of {0}?".to_string(),
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
                        template: "What is the capital of {0}?".to_string(),
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
        (
            "ContextualChoiceComponentMixed",
            ContextualChoiceComponentProps {
                challenge: ContextualChoice {
                    id: "mixed".to_string(),
                    name: "Mixed Placeholders Example".to_string(),
                    description: "Example with both types of placeholders".to_string(),
                    items: vec![ContextItem {
                        template: "Der Fluss {} fließt durch {1} und {0}.".to_string(),
                        choices: vec![
                            Choice {
                                id: 0,
                                correct_answer: "Deutschland".to_string(),
                                options: vec![
                                    "Deutschland".to_string(),
                                    "Österreich".to_string(),
                                    "Schweiz".to_string(),
                                ]
                            },
                            Choice {
                                id: 1,
                                correct_answer: "Österreich".to_string(),
                                options: vec![
                                    "Österreich".to_string(),
                                    "Deutschland".to_string(),
                                    "Schweiz".to_string(),
                                ]
                            },
                            Choice {
                                id: 2,
                                correct_answer: "Rhein".to_string(),
                                options: vec![
                                    "Rhein".to_string(),
                                    "Donau".to_string(),
                                    "Elbe".to_string(),
                                ]
                            }
                        ]
                    }],
                    ..Default::default()
                },
                on_command: None,
                on_event: None,
            }
        ),
        (
            "ContextualChoiceComponentNumbered",
            ContextualChoiceComponentProps {
                challenge: ContextualChoice {
                    id: "numbered".to_string(),
                    name: "Numbered Placeholders Example".to_string(),
                    description: "Example with numbered placeholders {0}, {1}, etc.".to_string(),
                    items: vec![ContextItem {
                        template: "Der {0} beschreibt die Erwärmung der Erde durch den Anstieg der {1} in der Atmosphäre. Wissenschaftler warnen, dass dies zu einem Anstieg des {2} führen könnte.".to_string(),
                        choices: vec![
                            Choice {
                                id: 0,
                                correct_answer: "Treibhauseffekt".to_string(),
                                options: vec![
                                    "Treibhauseffekt".to_string(),
                                    "Ozeanschutz".to_string(),
                                    "Küsteneffekt".to_string()
                                ]
                            },
                            Choice {
                                id: 1,
                                correct_answer: "Treibhausgase".to_string(),
                                options: vec![
                                    "Treibhausgase".to_string(),
                                    "Wassermoleküle".to_string(),
                                    "Sauerstoffwerte".to_string()
                                ]
                            },
                            Choice {
                                id: 2,
                                correct_answer: "Meeresspiegels".to_string(),
                                options: vec![
                                    "Meeresspiegels".to_string(),
                                    "Ozonlochs".to_string(),
                                    "Säuregehalts".to_string()
                                ]
                            }
                        ]
                    }],
                    ..Default::default()
                },
                on_command: None,
                on_event: None,
            }
        ),
        (
            "ContextualChoiceComponentUnnumbered",
            ContextualChoiceComponentProps {
                challenge: ContextualChoice {
                    id: "unnumbered".to_string(),
                    name: "Unnumbered Placeholders Example".to_string(),
                    description: "Example with unnumbered placeholders {}".to_string(),
                    items: vec![ContextItem {
                        template: "In Deutschland ist {} die Hauptstadt und {} die größte Stadt.".to_string(),
                        choices: vec![
                            Choice {
                                id: 0,
                                correct_answer: "Berlin".to_string(),
                                options: vec![
                                    "Berlin".to_string(),
                                    "München".to_string(),
                                    "Hamburg".to_string(),
                                    "Frankfurt".to_string()
                                ]
                            },
                            Choice {
                                id: 1,
                                correct_answer: "Hamburg".to_string(),
                                options: vec![
                                    "Hamburg".to_string(),
                                    "Berlin".to_string(),
                                    "Köln".to_string(),
                                    "München".to_string()
                                ]
                            }
                        ]
                    }],
                    ..Default::default()
                },
                on_command: None,
                on_event: None,
            }
        )
    );
}
