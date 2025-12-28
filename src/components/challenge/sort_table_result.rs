use konnektoren_core::challenges::{ChallengeResult, SortTable};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge: SortTable,
    pub challenge_result: ChallengeResult,
}

#[function_component(SortTableResultComponent)]
pub fn sort_table_result_component(props: &Props) -> Html {
    let results = match &props.challenge_result {
        ChallengeResult::SortTable(rows) => rows
            .iter()
            .zip(props.challenge.rows.iter())
            .map(|(row, row_result)| {
                let is_correct = row.values.eq(row_result.values.as_slice());
                let class_name = if is_correct {
                    "result-correct"
                } else {
                    "result-incorrect"
                };
                let text = format!("{} - {} - ", row.id, row.values.join(", "));

                html! {
                    <li class={class_name}>
                        <div class={class_name}>{text}<span>{
                            if is_correct {
                                "Correct"
                            } else {
                                "Incorrect"
                            }
                        }</span></div>

                    </li>
                }
            })
            .collect::<Vec<Html>>(),
        _ => panic!("Invalid challenge type"),
    };

    html! {
        <div class="challenge-result">
            <h2>{"Challenge Result"}</h2>
            <ul>
                {for results.into_iter()}
            </ul>

        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{SortTable, SortTableRow, sort_table::SortTableColumn};
    use yew_preview::prelude::*;

    fn create_test_challenge() -> SortTable {
        SortTable {
            id: "personal-pronouns".to_string(),
            name: "Personal Pronouns".to_string(),
            description: "Match the personal pronouns in their correct case".to_string(),
            columns: vec![
                SortTableColumn {
                    id: "nominativ".to_string(),
                    title: "Nominativ".to_string(),
                    description: "The subject of the sentence".to_string(),
                },
                SortTableColumn {
                    id: "akkusativ".to_string(),
                    title: "Akkusativ".to_string(),
                    description: "The direct object".to_string(),
                },
                SortTableColumn {
                    id: "dativ".to_string(),
                    title: "Dativ".to_string(),
                    description: "The indirect object".to_string(),
                },
            ],
            rows: vec![
                SortTableRow {
                    id: 0,
                    values: vec!["ich".to_string(), "mich".to_string(), "mir".to_string()],
                },
                SortTableRow {
                    id: 1,
                    values: vec!["du".to_string(), "dich".to_string(), "dir".to_string()],
                },
                SortTableRow {
                    id: 2,
                    values: vec![
                        "er/sie/es".to_string(),
                        "ihn/sie/es".to_string(),
                        "ihm/ihr/ihm".to_string(),
                    ],
                },
                SortTableRow {
                    id: 3,
                    values: vec!["wir".to_string(), "uns".to_string(), "uns".to_string()],
                },
            ],
        }
    }

    fn create_correct_result() -> ChallengeResult {
        ChallengeResult::SortTable(vec![
            SortTableRow {
                id: 0,
                values: vec!["ich".to_string(), "mich".to_string(), "mir".to_string()],
            },
            SortTableRow {
                id: 1,
                values: vec!["du".to_string(), "dich".to_string(), "dir".to_string()],
            },
            SortTableRow {
                id: 2,
                values: vec![
                    "er/sie/es".to_string(),
                    "ihn/sie/es".to_string(),
                    "ihm/ihr/ihm".to_string(),
                ],
            },
            SortTableRow {
                id: 3,
                values: vec!["wir".to_string(), "uns".to_string(), "uns".to_string()],
            },
        ])
    }

    fn create_incorrect_result() -> ChallengeResult {
        ChallengeResult::SortTable(vec![
            SortTableRow {
                id: 0,
                values: vec!["mich".to_string(), "ich".to_string(), "mir".to_string()], // Wrong order
            },
            SortTableRow {
                id: 1,
                values: vec!["dich".to_string(), "du".to_string(), "dir".to_string()], // Wrong order
            },
            SortTableRow {
                id: 2,
                values: vec![
                    "ihn/sie/es".to_string(),
                    "er/sie/es".to_string(),
                    "ihm/ihr/ihm".to_string(),
                ], // Wrong order
            },
            SortTableRow {
                id: 3,
                values: vec!["uns".to_string(), "wir".to_string(), "uns".to_string()], // Wrong order
            },
        ])
    }

    fn create_mixed_result() -> ChallengeResult {
        ChallengeResult::SortTable(vec![
            SortTableRow {
                id: 0,
                values: vec!["ich".to_string(), "mich".to_string(), "mir".to_string()], // Correct
            },
            SortTableRow {
                id: 1,
                values: vec!["dich".to_string(), "du".to_string(), "dir".to_string()], // Wrong
            },
            SortTableRow {
                id: 2,
                values: vec![
                    "er/sie/es".to_string(),
                    "ihn/sie/es".to_string(),
                    "ihm/ihr/ihm".to_string(),
                ], // Correct
            },
            SortTableRow {
                id: 3,
                values: vec!["uns".to_string(), "wir".to_string(), "uns".to_string()], // Wrong
            },
        ])
    }

    yew_preview::create_preview!(
        SortTableResultComponent,
        Props {
            challenge: create_test_challenge(),
            challenge_result: create_correct_result(),
        },
        (
            "All Incorrect",
            Props {
                challenge: create_test_challenge(),
                challenge_result: create_incorrect_result(),
            }
        ),
        (
            "Mixed Results",
            Props {
                challenge: create_test_challenge(),
                challenge_result: create_mixed_result(),
            }
        ),
        (
            "Empty Result",
            Props {
                challenge: create_test_challenge(),
                challenge_result: ChallengeResult::SortTable(vec![]),
            }
        )
    );
}
