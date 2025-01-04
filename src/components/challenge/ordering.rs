use super::{ChallengeActions, ChallengeActionsComponent};
use crate::components::ProgressBar;
use konnektoren_core::challenges::{ChallengeResult, Ordering, OrderingItem, OrderingResult};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::{ChallengeEvent, Event};
use rand::prelude::{thread_rng, SliceRandom};
use web_sys::DragEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OrderingComponentProps {
    pub challenge: Ordering,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
}

#[function_component(OrderingComponent)]
pub fn ordering_component(props: &OrderingComponentProps) -> Html {
    let current_item = use_state(|| 0);
    let dragged_index = use_state(|| None::<usize>);
    let drop_target_index = use_state(|| None::<usize>);
    // Initialize with shuffled order
    let current_order = use_state(|| {
        if let Some(item) = props.challenge.items.get(0) {
            let mut indices: Vec<usize> = (0..item.elements.len()).collect();
            use rand::seq::SliceRandom;
            use rand::thread_rng;
            indices.shuffle(&mut thread_rng());
            indices
        } else {
            vec![]
        }
    });
    let challenge_result = use_state(|| ChallengeResult::Ordering(vec![]));

    // Helper function to get ordered elements
    let get_ordered_elements = {
        let current_order = current_order.clone();
        move |elements: &[String]| -> Vec<String> {
            current_order
                .iter()
                .map(|&idx| elements[idx].clone())
                .collect()
        }
    };

    let handle_drag_start = {
        let dragged_index = dragged_index.clone();
        Callback::from(move |event: DragEvent| {
            if let Some(target) = event.target_dyn_into::<web_sys::Element>() {
                if let Ok(index) = target
                    .get_attribute("data-index")
                    .unwrap_or_default()
                    .parse()
                {
                    dragged_index.set(Some(index));
                }
            }
        })
    };

    let handle_drag_over = {
        let drop_target_index = drop_target_index.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            if let Some(target) = event.target_dyn_into::<web_sys::Element>() {
                if let Ok(index) = target
                    .get_attribute("data-index")
                    .unwrap_or_default()
                    .parse()
                {
                    drop_target_index.set(Some(index));
                }
            }
        })
    };

    let handle_drag_leave = {
        let drop_target_index = drop_target_index.clone();
        Callback::from(move |_: DragEvent| {
            drop_target_index.set(None);
        })
    };

    let handle_drop = {
        let current_order = current_order.clone();
        let dragged_index = dragged_index.clone();
        let drop_target_index = drop_target_index.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            if let Some(target) = event.target_dyn_into::<web_sys::Element>() {
                if let (Some(source_idx), Ok(target_idx)) = (
                    *dragged_index,
                    target
                        .get_attribute("data-index")
                        .unwrap_or_default()
                        .parse::<usize>(),
                ) {
                    // Swap positions in current_order
                    let mut new_order = (*current_order).clone();
                    new_order.swap(source_idx, target_idx);
                    current_order.set(new_order);
                }
            }
            dragged_index.set(None);
            drop_target_index.set(None);
        })
    };

    let handle_action = {
        let current_item = current_item.clone();
        let current_order = current_order.clone();
        let challenge = props.challenge.clone();
        let challenge_result = challenge_result.clone();
        let on_command = props.on_command.clone();
        let on_event = props.on_event.clone();

        Callback::from(move |action: ChallengeActions| match action {
            ChallengeActions::Next => {
                let next_index = *current_item + 1;
                let current_index = *current_item; // Get the current index as usize

                // Check if current order is correct and emit event
                if let Some(item) = challenge.items.get(current_index) {
                    if let Some(on_event) = on_event.as_ref() {
                        let is_correct = item.correct_order == *current_order;
                        if is_correct {
                            on_event.emit(Event::Challenge(ChallengeEvent::SolvedCorrect(
                                current_index,
                            )));
                        } else {
                            on_event.emit(Event::Challenge(ChallengeEvent::SolvedIncorrect(
                                current_index,
                            )));
                        }
                    }
                }

                // Save current result
                let result = OrderingResult {
                    order: (*current_order).clone(),
                };
                let mut results = match (*challenge_result).clone() {
                    ChallengeResult::Ordering(results) => results,
                    _ => vec![],
                };

                if results.len() <= current_index {
                    results.push(result);
                } else {
                    results[current_index] = result;
                }
                challenge_result.set(ChallengeResult::Ordering(results));

                if next_index < challenge.items.len() {
                    // Initialize order for next item
                    if let Some(next_item) = challenge.items.get(next_index) {
                        let mut indices: Vec<usize> = (0..next_item.elements.len()).collect();
                        indices.shuffle(&mut thread_rng());
                        current_order.set(indices);
                    }

                    // Move to next item
                    current_item.set(next_index);

                    if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::NextTask));
                    }
                } else if let Some(on_command) = on_command.as_ref() {
                    on_command.emit(Command::Challenge(ChallengeCommand::Finish(Some(
                        (*challenge_result).clone(),
                    ))));
                }
            }
            ChallengeActions::Previous => {
                if *current_item > 0 {
                    let prev_index = *current_item - 1;

                    // Initialize order for previous item
                    if let Some(prev_item) = challenge.items.get(prev_index) {
                        let mut indices: Vec<usize> = (0..prev_item.elements.len()).collect();
                        indices.shuffle(&mut thread_rng());
                        current_order.set(indices);
                    }

                    current_item.set(prev_index);

                    if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::PreviousTask));
                    }
                }
            }
            ChallengeActions::Help => {}
        })
    };

    let current_ordering_item = props.challenge.items.get(*current_item);
    if let Some(item) = current_ordering_item {
        let ordered_elements = get_ordered_elements(&item.elements);

        html! {
            <div class="ordering">
                <h2 class="ordering__title">{&props.challenge.name}</h2>
                <ProgressBar
                    value={*current_item}
                    max={props.challenge.items.len()}
                    label={format!("Item {} of {}", (*current_item) + 1, props.challenge.items.len())}
                />
                <div class="ordering__elements">
                    <div class="ordering__elements-list">
                        {ordered_elements.iter().enumerate().map(|(index, element)| {
                            let is_dragging = *dragged_index == Some(index);
                            let show_drop_indicator = *drop_target_index == Some(index);

                            html! {
                                <>
                                    if index > 0 {
                                        <div class={classes!(
                                            "ordering__drop-indicator",
                                            show_drop_indicator.then(|| "ordering__drop-indicator--active")
                                        )} />
                                    }
                                    <div
                                        class={classes!(
                                            "ordering__element",
                                            is_dragging.then(|| "ordering__element--dragging")
                                        )}
                                        draggable="true"
                                        data-index={index.to_string()}
                                        ondragstart={handle_drag_start.clone()}
                                        ondragover={handle_drag_over.clone()}
                                        ondragleave={handle_drag_leave.clone()}
                                        ondrop={handle_drop.clone()}
                                    >
                                        {element}
                                    </div>
                                </>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
                <ChallengeActionsComponent on_action={handle_action.clone()} />
            </div>
        }
    } else {
        html! {}
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn create_test_challenge() -> Ordering {
        Ordering {
            id: "test-ordering".to_string(),
            name: "Test Ordering Challenge".to_string(),
            description: "Order the elements correctly".to_string(),
            items: vec![
                OrderingItem {
                    elements: vec![
                        "First".to_string(),
                        "Second".to_string(),
                        "Third".to_string(),
                        "Fourth".to_string(),
                    ],
                    correct_order: vec![0, 1, 2, 3],
                },
                OrderingItem {
                    elements: vec![
                        "Apple".to_string(),
                        "Banana".to_string(),
                        "Cherry".to_string(),
                        "Date".to_string(),
                    ],
                    correct_order: vec![0, 1, 2, 3],
                },
            ],
        }
    }

    yew_preview::create_preview!(
        OrderingComponent,
        OrderingComponentProps {
            challenge: create_test_challenge(),
            on_command: None,
            on_event: None,
        },
    );
}
