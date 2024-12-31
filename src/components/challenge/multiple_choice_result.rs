use konnektoren_core::challenges::{ChallengeResult, MultipleChoice};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MultipleChoiceResultComponentProps {
    pub challenge: MultipleChoice,
    pub challenge_result: ChallengeResult,
}

#[function_component(MultipleChoiceResultComponent)]
pub fn multiple_choice_result_component(props: &MultipleChoiceResultComponentProps) -> Html {
    let results = match &props.challenge_result {
        ChallengeResult::MultipleChoice(options) => props
            .challenge
            .questions
            .iter()
            .zip(options.iter())
            .map(|(question, option)| {
                let is_correct = question.option == option.id;
                let modifier = if is_correct { "correct" } else { "incorrect" };
                let text = format!("{}: {} - ", question.question, option.name);

                html! {
                    <tr class={classes!("multiple-choice-result__row", format!("multiple-choice-result__row--{}", modifier))}>
                        <td class="multiple-choice-result__cell">
                            {text}
                        </td>
                        <td class={classes!("multiple-choice-result__cell", format!("multiple-choice-result__cell--{}", modifier))}>
                            {if is_correct { "Correct" } else { "Incorrect" }}
                        </td>
                    </tr>
                }
            })
            .collect::<Vec<Html>>(),
        _ => panic!("Invalid challenge type"),
    };

    html! {
        <div class="multiple-choice-result">
            <h2 class="multiple-choice-result__title">{"Challenge Result"}</h2>
            <table class="multiple-choice-result__table">
                <thead class="multiple-choice-result__header">
                    <tr>
                        <th class="multiple-choice-result__header-cell">{"Question"}</th>
                        <th class="multiple-choice-result__header-cell">{"Result"}</th>
                    </tr>
                </thead>
                <tbody class="multiple-choice-result__body">
                    {for results.into_iter()}
                </tbody>
            </table>
        </div>
    }
}
