use konnektoren_core::challenges::{ChallengeResult, Ordering};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OrderingResultComponentProps {
    pub challenge: Ordering,
    pub challenge_result: ChallengeResult,
}

#[function_component(OrderingResultComponent)]
pub fn ordering_result_component(props: &OrderingResultComponentProps) -> Html {
    let results = match &props.challenge_result {
        ChallengeResult::Ordering(results) => props
            .challenge
            .items
            .iter()
            .zip(results.iter())
            .map(|(item, result)| {
                let is_correct = item.correct_order == result.order;
                let modifier = if is_correct { "correct" } else { "incorrect" };

                let ordered_elements: Vec<String> = result
                    .order
                    .iter()
                    .map(|&idx| item.elements[idx].clone())
                    .collect();

                let correct_elements: Vec<String> = item
                    .correct_order
                    .iter()
                    .map(|&idx| item.elements[idx].clone())
                    .collect();

                html! {
                    <tr class={classes!("ordering-result__row", format!("ordering-result__row--{}", modifier))}>
                        <td class="ordering-result__cell">
                            {ordered_elements.join(" → ")}
                        </td>
                        <td class={classes!("ordering-result__cell", format!("ordering-result__cell--{}", modifier))}>
                            {if is_correct {
                                html! { "Correct" }
                            } else {
                                html! {
                                    <>
                                        {"Incorrect"}
                                        <div class="ordering-result__correct-answer">
                                            {"Correct order: "}
                                            {correct_elements.join(" → ")}
                                        </div>
                                    </>
                                }
                            }}
                        </td>
                    </tr>
                }
            })
            .collect::<Vec<Html>>(),
        _ => panic!("Invalid challenge result type"),
    };

    html! {
        <div class="ordering-result">
            <h2 class="ordering-result__title">{"Challenge Result"}</h2>
            <table class="ordering-result__table">
                <thead class="ordering-result__header">
                    <tr>
                        <th class="ordering-result__header-cell">{"Your Order"}</th>
                        <th class="ordering-result__header-cell">{"Result"}</th>
                    </tr>
                </thead>
                <tbody class="ordering-result__body">
                    {for results}
                </tbody>
            </table>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{OrderingItem, OrderingResult};
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

    fn create_test_result() -> ChallengeResult {
        ChallengeResult::Ordering(vec![
            OrderingResult {
                order: vec![1, 0, 2, 3], // Incorrect order
            },
            OrderingResult {
                order: vec![0, 1, 2, 3], // Correct order
            },
        ])
    }

    yew_preview::create_preview!(
        OrderingResultComponent,
        OrderingResultComponentProps {
            challenge: create_test_challenge(),
            challenge_result: create_test_result(),
        },
        (
            "Empty Result",
            OrderingResultComponentProps {
                challenge: create_test_challenge(),
                challenge_result: ChallengeResult::Ordering(vec![]),
            }
        ),
        (
            "All Correct",
            OrderingResultComponentProps {
                challenge: create_test_challenge(),
                challenge_result: ChallengeResult::Ordering(vec![
                    OrderingResult {
                        order: vec![0, 1, 2, 3],
                    },
                    OrderingResult {
                        order: vec![0, 1, 2, 3],
                    },
                ]),
            }
        ),
        (
            "All Incorrect",
            OrderingResultComponentProps {
                challenge: create_test_challenge(),
                challenge_result: ChallengeResult::Ordering(vec![
                    OrderingResult {
                        order: vec![3, 2, 1, 0],
                    },
                    OrderingResult {
                        order: vec![3, 2, 1, 0],
                    },
                ]),
            }
        ),
    );
}
