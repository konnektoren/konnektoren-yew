use crate::components::ChallengeTimerComponent;
use crate::i18n::use_i18n;
use konnektoren_core::challenges::{Challenge, ChallengeResult, Performance};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultSummaryComponentProps {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
}

#[function_component(ResultSummaryComponent)]
pub fn result_summary_component(props: &ResultSummaryComponentProps) -> Html {
    let i18n = use_i18n();
    let performance = props.challenge.performance(&props.challenge_result);

    let performance_modifier = match performance {
        p if p >= 90 => "excellent",
        p if p >= 70 => "good",
        p if p >= 50 => "fair",
        _ => "needs-improvement",
    };

    let performance_text = match performance {
        p if p >= 90 => i18n.t("Excellent!"),
        p if p >= 70 => i18n.t("Good job!"),
        p if p >= 50 => i18n.t("Fair attempt."),
        _ => i18n.t("Keep practicing!"),
    };

    html! {
        <details class="result-summary">
            <summary class={classes!("result-summary__header", format!("result-summary__header--{}", performance_modifier))}>
                <div class="result-summary__performance">
                    <span class="result-summary__score">{format!("{}%", performance)}</span>
                    <span class="result-summary__text">{performance_text}</span>
                </div>
            </summary>

            <div class="result-summary__content">

                <div class="result-summary__timer">
                    <ChallengeTimerComponent challenge={props.challenge.clone()} show_milliseconds={true} />
                </div>

                if performance > 50 {
                    <p class="result-summary__congratulation">
                        {i18n.t("Congratulations! You've completed the challenge successfully.")}
                    </p>
                }

                <p class="result-summary__message">
                    {i18n.t("Your performance shows your current understanding. Keep practicing to improve!")}
                </p>

                <div class="result-summary__details">
                    <p class="result-summary__challenge-name">
                        {&props.challenge.challenge_config.name}
                    </p>
                </div>
            </div>
        </details>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{Ordering, OrderingItem, OrderingResult};
    use konnektoren_core::prelude::{ChallengeConfig, ChallengeType};
    use yew_preview::prelude::*;

    fn create_test_challenge() -> Challenge {
        Challenge {
            challenge_type: ChallengeType::Ordering(Ordering {
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
            }),
            challenge_config: ChallengeConfig {
                id: "config-test".to_string(),
                name: "Test Challenge".to_string(),
                description: "Test Description".to_string(),
                challenge: "ordering".to_string(),
                variant: None,
                tasks: 2.into(),
                unlock_points: 10,
                position: None,
                icon: None,
            },
            challenge_result: ChallengeResult::default(),
            start_time: None,
            end_time: None,
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
        ResultSummaryComponent,
        ResultSummaryComponentProps {
            challenge: create_test_challenge(),
            challenge_result: create_test_result(),
        },
        (
            "Empty Result",
            ResultSummaryComponentProps {
                challenge: create_test_challenge(),
                challenge_result: ChallengeResult::Ordering(vec![]),
            }
        ),
        (
            "All Correct",
            ResultSummaryComponentProps {
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
            ResultSummaryComponentProps {
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
