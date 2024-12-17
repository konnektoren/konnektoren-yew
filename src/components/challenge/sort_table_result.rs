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
