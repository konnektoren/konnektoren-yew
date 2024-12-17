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
                let class_name = if is_correct {
                    "result-correct"
                } else {
                    "result-incorrect"
                };
                let text = format!("{}: {} - ", question.question, option.name);

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
