use konnektoren_core::challenges::{ChallengeResult, GapFill, GapFillAnswer};
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
