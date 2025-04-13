use konnektoren_core::challenges::{ChallengeResult, ContextualChoice};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContextualChoiceResultComponentProps {
    pub challenge: ContextualChoice,
    pub challenge_result: ChallengeResult,
}

#[function_component(ContextualChoiceResultComponent)]
pub fn contextual_choice_result_component(props: &ContextualChoiceResultComponentProps) -> Html {
    let results = match &props.challenge_result {
        ChallengeResult::ContextualChoice(answers) => props
            .challenge
            .items
            .iter()
            .zip(answers.iter())
            .map(|(item, answer)| {
                // Directly use the 0-based indices from the selection component
                let filled_template = fill_template(&item.template, &item.choices, &answer.ids);

                // Check correctness with 0-based indices
                let is_correct = item.choices.iter().zip(&answer.ids).all(|(choice, &id)| {
                    choice.options.get(id).map_or(false, |selected| *selected == choice.correct_answer)
                });

                let class_name = if is_correct {
                    "result-correct"
                } else {
                    "result-incorrect"
                };

                // Calculate indices of correct answers (0-based)
                let correct_indices: Vec<usize> = item.choices
                    .iter()
                    .map(|c| c.options.iter().position(|o| o == &c.correct_answer).unwrap_or(0))
                    .collect();

                html! {
                    <li class={class_name}>
                        <div class={class_name}>
                            {filled_template}
                            <span>{
                                if is_correct {
                                    " - Correct"
                                } else {
                                    " - Incorrect"
                                }
                            }</span>
                        </div>
                        {
                            if !is_correct {
                                html! {
                                    <div class="correct-answer">
                                        {"Correct answer: "}
                                        {fill_template(&item.template, &item.choices, &correct_indices)}
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </li>
                }
            })
            .collect::<Vec<Html>>(),
        _ => panic!("Invalid challenge result type"),
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

fn fill_template(
    template: &str,
    choices: &[konnektoren_core::challenges::Choice],
    selected_ids: &[usize],
) -> String {
    use regex::Regex;

    // Handle both numbered and unnumbered placeholders
    let re = Regex::new(r"\{(\d*)\}").unwrap();
    let mut result = template.to_string();

    // First pass: identify which choice indices are explicitly referenced
    let mut used_indices = std::collections::HashSet::new();
    for cap in re.captures_iter(template) {
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
    let mut available_indices: Vec<usize> = (0..choices.len())
        .filter(|idx| !used_indices.contains(idx))
        .collect();

    // Replace all placeholders in the template
    let mut unnumbered_count = 0;

    // Use a temporary copy for the captures iterator
    let temp_template = result.clone();

    for cap in re.captures_iter(&temp_template) {
        let whole_match = cap.get(0).unwrap().as_str();

        // Determine the choice index to use
        let choice_index = if let Some(digit_match) = cap.get(1) {
            let digit_str = digit_match.as_str();
            if digit_str.is_empty() {
                // For {} (unnumbered), use the next available index
                if let Some(idx) = available_indices.get(unnumbered_count) {
                    unnumbered_count += 1;
                    *idx
                } else {
                    // If we've used all available indices, just use index 0
                    0
                }
            } else {
                // For {0}, {1}, etc., use the specified index
                digit_str.parse().unwrap_or(0)
            }
        } else {
            // This shouldn't happen with our regex, but just in case
            0
        };

        if choice_index < choices.len() && choice_index < selected_ids.len() {
            let selected_id = selected_ids[choice_index];
            if selected_id < choices[choice_index].options.len() {
                let replacement = &choices[choice_index].options[selected_id];
                // Replace just this placeholder with the selected option
                result = result.replacen(whole_match, replacement, 1);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::challenges::Choice;

    #[test]
    fn test_simple_unnumbered_placeholder() {
        let template = "Wir haben {} Zeit.";
        let choices = vec![Choice {
            id: 0,
            options: vec!["kein".to_string(), "nicht".to_string(), "keine".to_string()],
            correct_answer: "keine".to_string(),
        }];
        let selected_ids = vec![2]; // Selecting "keine" (0-based index)

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(result, "Wir haben keine Zeit.");
    }

    #[test]
    fn test_multiple_unnumbered_placeholders() {
        let template = "In {} ist {} die Hauptstadt.";
        let choices = vec![
            Choice {
                id: 0,
                options: vec![
                    "Deutschland".to_string(),
                    "Frankreich".to_string(),
                    "Italien".to_string(),
                ],
                correct_answer: "Deutschland".to_string(),
            },
            Choice {
                id: 1,
                options: vec!["Berlin".to_string(), "Paris".to_string(), "Rom".to_string()],
                correct_answer: "Berlin".to_string(),
            },
        ];
        let selected_ids = vec![0, 0]; // Deutschland, Berlin (0-based indices)

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(result, "In Deutschland ist Berlin die Hauptstadt.");
    }

    #[test]
    fn test_numbered_placeholders() {
        let template = "Der {0} fließt durch {1}.";
        let choices = vec![
            Choice {
                id: 0,
                options: vec!["Rhein".to_string(), "Donau".to_string(), "Elbe".to_string()],
                correct_answer: "Rhein".to_string(),
            },
            Choice {
                id: 1,
                options: vec![
                    "Deutschland".to_string(),
                    "Österreich".to_string(),
                    "Schweiz".to_string(),
                ],
                correct_answer: "Deutschland".to_string(),
            },
        ];
        let selected_ids = vec![0, 0]; // Rhein, Deutschland (0-based indices)

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(result, "Der Rhein fließt durch Deutschland.");
    }

    #[test]
    fn test_mixed_placeholders() {
        let template = "Der Fluss {} fließt durch {1} und {0}.";
        let choices = vec![
            Choice {
                id: 0,
                options: vec!["Deutschland".to_string(), "Österreich".to_string()],
                correct_answer: "Deutschland".to_string(),
            },
            Choice {
                id: 1,
                options: vec!["Österreich".to_string(), "Deutschland".to_string()],
                correct_answer: "Österreich".to_string(),
            },
            Choice {
                id: 2,
                options: vec!["Rhein".to_string(), "Donau".to_string()],
                correct_answer: "Rhein".to_string(),
            },
        ];
        let selected_ids = vec![0, 0, 0]; // Deutschland, Österreich, Rhein (0-based indices)

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(
            result,
            "Der Fluss Rhein fließt durch Österreich und Deutschland."
        );
    }

    #[test]
    fn test_out_of_order_numbered_placeholders() {
        let template = "Person {2} und Person {0} besuchen {1}.";
        let choices = vec![
            Choice {
                id: 0,
                options: vec!["Anna".to_string(), "Karl".to_string()],
                correct_answer: "Anna".to_string(),
            },
            Choice {
                id: 1,
                options: vec!["Berlin".to_string(), "Hamburg".to_string()],
                correct_answer: "Berlin".to_string(),
            },
            Choice {
                id: 2,
                options: vec!["Max".to_string(), "Lisa".to_string()],
                correct_answer: "Max".to_string(),
            },
        ];
        let selected_ids = vec![0, 0, 0]; // Anna, Berlin, Max (0-based indices)

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(result, "Person Max und Person Anna besuchen Berlin.");
    }

    #[test]
    fn test_selected_id_out_of_bounds() {
        let template = "Dies ist ein {0} mit einem {1} Index.";
        let choices = vec![
            Choice {
                id: 0,
                options: vec!["Test".to_string(), "Satz".to_string()],
                correct_answer: "Test".to_string(),
            },
            Choice {
                id: 1,
                options: vec!["gültigen".to_string(), "ungültigen".to_string()],
                correct_answer: "gültigen".to_string(),
            },
        ];
        // Selecting indices outside the options array
        let selected_ids = vec![3, 4];

        // Should not crash, but may produce incomplete template
        let result = fill_template(template, &choices, &selected_ids);
        // The result might not replace the placeholders if indices are invalid
        assert!(result.contains("{0}") || result.contains("{1}"));
    }

    #[test]
    fn test_missing_selected_ids() {
        let template = "Dies ist ein {0} mit {1} und {2}.";
        let choices = vec![
            Choice {
                id: 0,
                options: vec!["Test".to_string(), "Satz".to_string()],
                correct_answer: "Test".to_string(),
            },
            Choice {
                id: 1,
                options: vec!["einem".to_string(), "zwei".to_string()],
                correct_answer: "einem".to_string(),
            },
            Choice {
                id: 2,
                options: vec!["Platzhaltern".to_string(), "Wörtern".to_string()],
                correct_answer: "Platzhaltern".to_string(),
            },
        ];
        // Fewer selected_ids than choices
        let selected_ids = vec![0, 0];

        let result = fill_template(template, &choices, &selected_ids);
        // The third placeholder should remain unreplaced
        assert!(result.contains("Test") && result.contains("einem") && result.contains("{2}"));
    }

    #[test]
    fn test_high_numbered_placeholders() {
        let template = "Ein Test mit {10} Platzhalter.";
        let choices = vec![
            Choice {
                id: 0,
                options: vec!["einem".to_string(), "keinem".to_string()],
                correct_answer: "einem".to_string(),
            },
            // Add 10 more dummy choices to have at least 11 choices
            Choice {
                id: 1,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 2,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 3,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 4,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 5,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 6,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 7,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 8,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 9,
                options: vec!["".to_string()],
                correct_answer: "".to_string(),
            },
            Choice {
                id: 10,
                options: vec!["hohem".to_string(), "niedrigem".to_string()],
                correct_answer: "hohem".to_string(),
            },
        ];
        // Set up enough IDs (11 total)
        let mut selected_ids = vec![0; 11];
        selected_ids[10] = 0; // "hohem" for the 11th choice (0-based index)

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(result, "Ein Test mit hohem Platzhalter.");
    }

    #[test]
    fn test_repeated_placeholder() {
        let template = "Dies ist {0} und noch einmal {0}.";
        let choices = vec![Choice {
            id: 0,
            options: vec!["ein Test".to_string(), "ein Beispiel".to_string()],
            correct_answer: "ein Test".to_string(),
        }];
        let selected_ids = vec![0]; // "ein Test" (0-based index)

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(result, "Dies ist ein Test und noch einmal ein Test.");
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{
        Choice, ContextItem, ContextItemChoiceAnswers, ContextualChoice,
    };
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ContextualChoiceResultComponent,
        ContextualChoiceResultComponentProps {
            challenge: ContextualChoice {
                id: "default".to_string(),
                name: "Default Example".to_string(),
                description: "Default example with correct answers".to_string(),
                items: vec![
                    ContextItem {
                        template: "Der {0} beschreibt die Erwärmung der Erde durch den Anstieg der {1} in der Atmosphäre.".to_string(),
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
                            }
                        ]
                    }
                ]
            },
            challenge_result: ChallengeResult::ContextualChoice(vec![
                ContextItemChoiceAnswers {
                    ids: vec![0, 0]  // Correct: "Treibhauseffekt", "Treibhausgase" (0-based indices)
                }
            ])
        },
        (
            "Incorrect",
            ContextualChoiceResultComponentProps {
                challenge: ContextualChoice {
                    id: "incorrect".to_string(),
                    name: "Incorrect Example".to_string(),
                    description: "Example with incorrect answers".to_string(),
                    items: vec![
                        ContextItem {
                            template: "Der {0} beschreibt die Erwärmung der Erde durch den Anstieg der {1} in der Atmosphäre.".to_string(),
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
                                }
                            ]
                        }
                    ]
                },
                challenge_result: ChallengeResult::ContextualChoice(vec![
                    ContextItemChoiceAnswers {
                        ids: vec![1, 1]  // Incorrect: "Ozeanschutz", "Wassermoleküle" (0-based indices)
                    }
                ])
            }
        ),
        (
            "Unnumbered",
            ContextualChoiceResultComponentProps {
                challenge: ContextualChoice {
                    id: "unnumbered".to_string(),
                    name: "Unnumbered Example".to_string(),
                    description: "Example with unnumbered placeholders".to_string(),
                    items: vec![
                        ContextItem {
                            template: "In Deutschland ist {} die Hauptstadt und {} die größte Stadt.".to_string(),
                            choices: vec![
                                Choice {
                                    id: 0,
                                    correct_answer: "Berlin".to_string(),
                                    options: vec![
                                        "Berlin".to_string(),
                                        "München".to_string(),
                                        "Hamburg".to_string()
                                    ]
                                },
                                Choice {
                                    id: 1,
                                    correct_answer: "Hamburg".to_string(),
                                    options: vec![
                                        "Hamburg".to_string(),
                                        "Berlin".to_string(),
                                        "München".to_string()
                                    ]
                                }
                            ]
                        }
                    ]
                },
                challenge_result: ChallengeResult::ContextualChoice(vec![
                    ContextItemChoiceAnswers {
                        ids: vec![0, 0]  // Correct: "Berlin", "Hamburg" (0-based indices)
                    }
                ])
            }
        ),
        (
            "Mixed",
            ContextualChoiceResultComponentProps {
                challenge: ContextualChoice {
                    id: "mixed".to_string(),
                    name: "Mixed Example".to_string(),
                    description: "Example with mixed placeholders".to_string(),
                    items: vec![
                        ContextItem {
                            template: "Der Fluss {} fließt durch {1} und {0}.".to_string(),
                            choices: vec![
                                Choice {
                                    id: 0,
                                    correct_answer: "Deutschland".to_string(),
                                    options: vec![
                                        "Deutschland".to_string(),
                                        "Österreich".to_string(),
                                        "Schweiz".to_string()
                                    ]
                                },
                                Choice {
                                    id: 1,
                                    correct_answer: "Österreich".to_string(),
                                    options: vec![
                                        "Österreich".to_string(),
                                        "Deutschland".to_string(),
                                        "Schweiz".to_string()
                                    ]
                                },
                                Choice {
                                    id: 2,
                                    correct_answer: "Rhein".to_string(),
                                    options: vec![
                                        "Rhein".to_string(),
                                        "Donau".to_string(),
                                        "Elbe".to_string()
                                    ]
                                }
                            ]
                        }
                    ]
                },
                challenge_result: ChallengeResult::ContextualChoice(vec![
                    ContextItemChoiceAnswers {
                        ids: vec![0, 0, 0]  // "Deutschland", "Österreich", "Rhein" (0-based indices)
                    }
                ])
            }
        ),
        (
            "WasserMangelExample",
            ContextualChoiceResultComponentProps {
                challenge: ContextualChoice {
                    id: "wassermangel".to_string(),
                    name: "Wassermangel Example".to_string(),
                    description: "Example with the Wassermangel example".to_string(),
                    items: vec![
                        ContextItem {
                            template: "Der {0} beschreibt eine lange Zeit ohne Niederschlag, die zu {1} führen kann. In manchen Regionen der Welt führt dies zu {2} und Hungersnöten.".to_string(),
                            choices: vec![
                                Choice {
                                    id: 0,
                                    options: vec![
                                        "Wassermangel".to_string(),
                                        "Frosteintritt".to_string(),
                                        "Taifun".to_string()
                                    ],
                                    correct_answer: "Wassermangel".to_string(),
                                },
                                Choice {
                                    id: 1,
                                    options: vec![
                                        "Dürre".to_string(),
                                        "Überschwemmung".to_string(),
                                        "Vulkanausbruch".to_string()
                                    ],
                                    correct_answer: "Dürre".to_string(),
                                },
                                Choice {
                                    id: 2,
                                    options: vec![
                                        "Wasserknappheit".to_string(),
                                        "Luftverschmutzung".to_string(),
                                        "Bodenerosion".to_string()
                                    ],
                                    correct_answer: "Wasserknappheit".to_string(),
                                }
                            ]
                        }
                    ]
                },
                challenge_result: ChallengeResult::ContextualChoice(vec![
                    ContextItemChoiceAnswers {
                        ids: vec![1, 1, 1]  // Incorrect: "Frosteintritt", "Überschwemmung", "Luftverschmutzung" (0-based indices)
                    }
                ])
            }
        )
    );
}
