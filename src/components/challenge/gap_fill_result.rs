use konnektoren_core::challenges::{ChallengeResult, GapFill};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GapFillResultComponentProps {
    pub challenge: GapFill,
    pub challenge_result: ChallengeResult,
}

#[function_component(GapFillResultComponent)]
pub fn gap_fill_result_component(props: &GapFillResultComponentProps) -> Html {
    let results = match &props.challenge_result {
        ChallengeResult::GapFill(answers) => answers
            .iter()
            .filter_map(|answer| {
                props.challenge.questions.get(answer.question_index).map(|question| {
                    let is_correct = props.challenge.check_answer(answer);
                    let modifier = if is_correct { "correct" } else { "incorrect" };

                    // Create filled sentence with answers
                    let parts: Vec<&str> = question.sentence.split("__").collect();
                    let filled_sentence = parts
                        .iter()
                        .enumerate()
                        .map(|(i, part)| {
                            if i < answer.answers.len() {
                                format!("{}{}", part, answer.answers[i])
                            } else {
                                part.to_string()
                            }
                        })
                        .collect::<String>();

                    // Get correct answers for comparison
                    let correct_answers = question
                        .gaps
                        .iter()
                        .map(|gap| gap.correct.clone())
                        .collect::<Vec<String>>()
                        .join(", ");

                    html! {
                        <tr class={classes!("gap-fill-result__row", format!("gap-fill-result__row--{}", modifier))}>
                            <td class="gap-fill-result__cell">
                                {filled_sentence}
                            </td>
                            <td class={classes!("gap-fill-result__cell", format!("gap-fill-result__cell--{}", modifier))}>
                                {if is_correct {
                                    "Correct".to_string()
                                } else {
                                    format!("Incorrect (Correct: {})", correct_answers)
                                }}
                            </td>
                        </tr>
                    }
                })
            })
            .collect::<Vec<Html>>(),
        _ => panic!("Invalid challenge type"),
    };

    html! {
        <div class="gap-fill-result">
            <h2 class="gap-fill-result__title">{"Challenge Result"}</h2>
            <table class="gap-fill-result__table">
                <thead class="gap-fill-result__header">
                    <tr>
                        <th class="gap-fill-result__header-cell">{"Sentence"}</th>
                        <th class="gap-fill-result__header-cell">{"Result"}</th>
                    </tr>
                </thead>
                <tbody class="gap-fill-result__body">
                    {for results.into_iter()}
                </tbody>
            </table>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{Gap, GapFillAnswer, GapFillQuestion};
    use yew_preview::prelude::*;

    fn create_test_challenge() -> GapFill {
        GapFill {
            id: "gap-fill-test".to_string(),
            name: "Past Tense Test".to_string(),
            description: "Fill in the correct past tense forms".to_string(),
            lang: "de".to_string(),
            questions: vec![
                GapFillQuestion {
                    sentence: "Ich __ nach Berlin __ (fahren).".to_string(),
                    gaps: vec![
                        Gap {
                            position: 0,
                            options: vec!["bin".to_string(), "habe".to_string(), "war".to_string()],
                            correct: "bin".to_string(),
                        },
                        Gap {
                            position: 1,
                            options: vec![
                                "gefahren".to_string(),
                                "gefahrt".to_string(),
                                "fuhr".to_string(),
                            ],
                            correct: "gefahren".to_string(),
                        },
                    ],
                    hints: vec!["Movement verbs use 'sein' as auxiliary".to_string()],
                    translation: "I went to Berlin".to_string(),
                    explanation: "We use 'sein' with verbs of movement".to_string(),
                },
                GapFillQuestion {
                    sentence: "Er __ gestern viel __ (arbeiten).".to_string(),
                    gaps: vec![
                        Gap {
                            position: 0,
                            options: vec!["hat".to_string(), "ist".to_string(), "wird".to_string()],
                            correct: "hat".to_string(),
                        },
                        Gap {
                            position: 1,
                            options: vec![
                                "gearbeitet".to_string(),
                                "arbeitet".to_string(),
                                "arbeitete".to_string(),
                            ],
                            correct: "gearbeitet".to_string(),
                        },
                    ],
                    hints: vec!["Regular verbs use 'haben' in perfect tense".to_string()],
                    translation: "He worked a lot yesterday".to_string(),
                    explanation: "Regular verbs form their perfect tense with 'haben'".to_string(),
                },
            ],
        }
    }

    fn create_correct_result() -> ChallengeResult {
        ChallengeResult::GapFill(vec![
            GapFillAnswer {
                question_index: 0,
                answers: vec!["bin".to_string(), "gefahren".to_string()],
            },
            GapFillAnswer {
                question_index: 1,
                answers: vec!["hat".to_string(), "gearbeitet".to_string()],
            },
        ])
    }

    fn create_incorrect_result() -> ChallengeResult {
        ChallengeResult::GapFill(vec![
            GapFillAnswer {
                question_index: 0,
                answers: vec!["habe".to_string(), "gefahrt".to_string()], // Wrong answers
            },
            GapFillAnswer {
                question_index: 1,
                answers: vec!["ist".to_string(), "arbeitet".to_string()], // Wrong answers
            },
        ])
    }

    fn create_mixed_result() -> ChallengeResult {
        ChallengeResult::GapFill(vec![
            GapFillAnswer {
                question_index: 0,
                answers: vec!["bin".to_string(), "gefahren".to_string()], // Correct
            },
            GapFillAnswer {
                question_index: 1,
                answers: vec!["ist".to_string(), "arbeitet".to_string()], // Wrong
            },
        ])
    }

    yew_preview::create_preview!(
        GapFillResultComponent,
        GapFillResultComponentProps {
            challenge: create_test_challenge(),
            challenge_result: create_correct_result(),
        },
        (
            "All Incorrect",
            GapFillResultComponentProps {
                challenge: create_test_challenge(),
                challenge_result: create_incorrect_result(),
            }
        ),
        (
            "Mixed Results",
            GapFillResultComponentProps {
                challenge: create_test_challenge(),
                challenge_result: create_mixed_result(),
            }
        ),
        (
            "Empty Result",
            GapFillResultComponentProps {
                challenge: create_test_challenge(),
                challenge_result: ChallengeResult::GapFill(vec![]),
            }
        )
    );
}
