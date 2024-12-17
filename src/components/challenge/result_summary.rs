use crate::components::ChallengeTimerComponent;
use konnektoren_core::challenges::{Challenge, ChallengeResult, Performance};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultSummaryComponentProps {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
}

#[function_component(ResultSummaryComponent)]
pub fn result_summary_component(props: &ResultSummaryComponentProps) -> Html {
    let performance = props.challenge.performance(&props.challenge_result);

    let (performance_class, performance_text) = match performance {
        p if p >= 90 => ("performance-excellent", "Excellent!"),
        p if p >= 70 => ("performance-good", "Good job!"),
        p if p >= 50 => ("performance-fair", "Fair attempt."),
        _ => ("performance-needs-improvement", "Keep practicing!"),
    };

    html! {
        <section class="result-summary material-card">
            <h2 class="material-text--h4">{"Challenge Result"}</h2>
            <div class="result-content">
                <ChallengeTimerComponent challenge={props.challenge.clone()} show_milliseconds={true} />
                if performance > 50 {
                    <p class="congratulation material-text--body1">{"Congratulations! You've completed the challenge successfully."}</p>
                }
                <div class={classes!("performance", performance_class)}>
                    <span class="performance-score material-text--h5">{format!("{}%", performance)}</span>
                    <span class="performance-text material-text--body2">{performance_text}</span>
                </div>
                <p class="material-text--body2">{"Your performance shows your current understanding. Keep practicing to improve!"}</p>
            </div>
        </section>
    }
}
