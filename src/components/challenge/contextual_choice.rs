use super::{ChallengeActions, ChallengeActionsComponent};
use crate::components::ProgressBar;
use crate::i18n::use_i18n;
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

    #[derive(PartialEq)]
    struct EffectDeps {
        selections: HashMap<(usize, usize), usize>,
        item_index: usize,
    }

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

                let all_options_selected = current_item
                    .choices
                    .iter()
                    .enumerate()
                    .all(|(idx, _)| deps.selections.get(&(deps.item_index, idx)).is_some());

                if all_options_selected {
                    let all_correct =
                        current_item
                            .choices
                            .iter()
                            .enumerate()
                            .all(|(choice_idx, choice)| {
                                if let Some(&selected_idx) =
                                    deps.selections.get(&(deps.item_index, choice_idx))
                                {
                                    selected_idx < choice.options.len()
                                        && choice.options[selected_idx] == choice.correct_answer
                                } else {
                                    false
                                }
                            });

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

                    if deps.item_index < challenge.items.len() - 1 {
                        let next_idx = deps.item_index + 1;
                        let mut new_selections = deps.selections.clone();
                        new_selections.retain(|&(item, _), _| item != next_idx);
                        selections.set(new_selections);
                        item_index.set(next_idx);

                        if let Some(cmd) = &on_command {
                            cmd.emit(Command::Challenge(ChallengeCommand::NextTask));
                        }
                    } else if all_correct {
                        if let Some(cmd) = &on_command {
                            cmd.emit(Command::Challenge(ChallengeCommand::Finish(Some(
                                (*challenge_result).clone(),
                            ))));
                        }
                    }
                }
            }
            || {}
        });
    }

    let handle_option_selection = {
        let selections = selections.clone();
        let challenge = props.challenge.clone();
        let challenge_result = challenge_result.clone();
        let item_index = item_index.clone();

        Callback::from(move |(choice_index, option_index): (usize, usize)| {
            #[cfg(feature = "csr")]
            {
                use konnektoren_core::challenges::ContextItemChoiceAnswers;
                let mut new_selections = (*selections).clone();
                new_selections.insert((*item_index, choice_index), option_index);

                let item_choices = &challenge.items[*item_index].choices;
                let ids: Vec<usize> = item_choices
                    .iter()
                    .enumerate()
                    .map(|(choice_index, _)| {
                        new_selections
                            .get(&(*item_index, choice_index))
                            .cloned()
                            .unwrap_or(0)
                    })
                    .collect();

                selections.set(new_selections);

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
            <ContextualChoiceProgress
                item_index={*item_index}
                total_items={props.challenge.items.len()}
            />
            <div class="contextual-choice__item">
                <ContextualChoiceTemplate
                    current_item={current_item.clone()}
                    selections={(*selections).clone()}
                    item_index={*item_index}
                    handle_option_selection={handle_option_selection.clone()}
                />
            </div>
            if *show_help {
                <ContextualChoiceHelp current_item={current_item.clone()} />
            }
            <ChallengeActionsComponent on_action={handle_action} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextualChoiceProgressProps {
    pub item_index: usize,
    pub total_items: usize,
}

#[function_component(ContextualChoiceProgress)]
fn contextual_choice_progress(props: &ContextualChoiceProgressProps) -> Html {
    let i18n = use_i18n();
    html! {
        <div class="contextual-choice__progress">
            <ProgressBar
                value={props.item_index}
                max={props.total_items}
                label={format!("{} {} {} {}", i18n.t("Item"), props.item_index + 1, i18n.t("of"), props.total_items)}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextualChoiceHelpProps {
    pub current_item: konnektoren_core::challenges::ContextItem,
}

#[function_component(ContextualChoiceHelp)]
fn contextual_choice_help(props: &ContextualChoiceHelpProps) -> Html {
    let i18n = use_i18n();
    html! {
        <div class="contextual-choice__help">
            <h3 class="contextual-choice__help-title">{ i18n.t("Help") }</h3>
            <p class="contextual-choice__help-text">
                { i18n.t("Fill in the blanks by selecting the appropriate option for each dropdown.") }
            </p>
            <div class="contextual-choice__help-hints">
                <h4 class="contextual-choice__help-hints-title">{ i18n.t("Hints:") }</h4>
                <ul>
                    {
                        props.current_item.choices.iter().map(|choice| {
                            html! {
                                <li class="contextual-choice__help-hint">
                                    { i18n.t("The correct answer is: ") }
                                    <strong>{ &choice.correct_answer }</strong>
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextualChoiceTemplateProps {
    pub current_item: konnektoren_core::challenges::ContextItem,
    pub selections: HashMap<(usize, usize), usize>,
    pub item_index: usize,
    pub handle_option_selection: Callback<(usize, usize)>,
}

#[function_component(ContextualChoiceTemplate)]
fn contextual_choice_template(props: &ContextualChoiceTemplateProps) -> Html {
    let re = regex::Regex::new(r"\{(\d*)\}").unwrap();

    let mut used_indices = std::collections::HashSet::new();
    for cap in re.captures_iter(&props.current_item.template) {
        if let Some(digit_match) = cap.get(1) {
            let digit_str = digit_match.as_str();
            if !digit_str.is_empty() {
                if let Ok(idx) = digit_str.parse::<usize>() {
                    used_indices.insert(idx);
                }
            }
        }
    }

    let mut available_indices: Vec<usize> = (0..props.current_item.choices.len())
        .filter(|idx| !used_indices.contains(idx))
        .collect();

    let mut last_end = 0;
    let mut result = Vec::new();

    for cap in re.captures_iter(&props.current_item.template) {
        let whole_match = cap.get(0).unwrap();

        if whole_match.start() > last_end {
            result.push(html! {
                <span>{ &props.current_item.template[last_end..whole_match.start()] }</span>
            });
        }

        let choice_index = if let Some(digit_match) = cap.get(1) {
            let digit_str = digit_match.as_str();
            if digit_str.is_empty() {
                available_indices.pop().unwrap_or(0)
            } else {
                digit_str.parse().unwrap_or(0)
            }
        } else {
            0
        };

        if choice_index < props.current_item.choices.len() {
            result.push(html! {
                <ContextualChoiceSelect
                    choice_index={choice_index}
                    current_item={props.current_item.clone()}
                    selections={props.selections.clone()}
                    item_index={props.item_index}
                    handle_option_selection={props.handle_option_selection.clone()}
                />
            });
        }

        last_end = whole_match.end();
    }

    if last_end < props.current_item.template.len() {
        result.push(html! {
            <span>{ &props.current_item.template[last_end..] }</span>
        });
    }

    html! { <>{for result}</> }
}

#[derive(Properties, PartialEq)]
pub struct ContextualChoiceSelectProps {
    pub choice_index: usize,
    pub current_item: konnektoren_core::challenges::ContextItem,
    pub selections: HashMap<(usize, usize), usize>,
    pub item_index: usize,
    pub handle_option_selection: Callback<(usize, usize)>,
}

#[function_component(ContextualChoiceSelect)]
fn contextual_choice_select(props: &ContextualChoiceSelectProps) -> Html {
    let i18n = use_i18n();
    let selected_value = props
        .selections
        .get(&(props.item_index, props.choice_index))
        .cloned();
    let select_id = format!("choice-select-{}-{}", props.item_index, props.choice_index);

    #[cfg(feature = "csr")]
    let onchange = {
        let handle_option_selection = props.handle_option_selection.clone();
        let choice_index = props.choice_index;
        Callback::from(move |e: yew::Event| {
            use wasm_bindgen::JsCast;
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
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
            key={select_id.clone()}
            onchange={onchange}
        >
            <option value="" disabled=true selected={selected_value.is_none()}>
                { i18n.t("Select an option") }
            </option>
            {
                props.current_item.choices[props.choice_index].options.iter().enumerate().map(|(j, option)| {
                    html! {
                        <option
                            value={j.to_string()}
                            selected={selected_value == Some(j)}
                        >
                            { option }
                        </option>
                    }
                }).collect::<Html>()
            }
        </select>
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
