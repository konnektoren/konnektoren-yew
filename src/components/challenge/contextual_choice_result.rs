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
                let filled_template = fill_template(&item.template, &item.choices, &answer.ids);
                let is_correct = item.choices.iter().zip(&answer.ids).all(|(choice, &id)| {
                    choice.options.get(id).map_or(false, |selected| *selected == choice.correct_answer)
                });
                let class_name = if is_correct {
                    "result-correct"
                } else {
                    "result-incorrect"
                };

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
                                        {fill_template(&item.template, &item.choices, &item.choices.iter().map(|c| c.options.iter().position(|o| o == &c.correct_answer).unwrap()).collect::<Vec<_>>())}
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
    let mut result = template.to_string();
    for (i, &id) in selected_ids.iter().enumerate() {
        let placeholder = "{}";
        let replacement = &choices[i].options[id];
        result = result.replacen(placeholder, replacement, 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::challenges::Choice;

    #[test]
    fn test_fill_template() {
        let template = "Wir haben {} Zeit.";
        let choices = vec![Choice {
            id: 0,
            options: vec!["kein".to_string(), "nicht".to_string(), "keine".to_string()],
            correct_answer: "keine".to_string(),
        }];
        let selected_ids = vec![2]; // Selecting "keine"

        let result = fill_template(template, &choices, &selected_ids);
        assert_eq!(result, "Wir haben keine Zeit.");
    }
}
