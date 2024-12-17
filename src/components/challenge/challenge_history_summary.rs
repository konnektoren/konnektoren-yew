use konnektoren_core::challenges::ChallengeHistory;
use konnektoren_core::prelude::{Challenge, Performance};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeHistorySummaryProps {
    pub challenge_history: ChallengeHistory,
}

#[function_component(ChallengeHistorySummaryComponent)]
pub fn challenge_history_summary(props: &ChallengeHistorySummaryProps) -> Html {
    html! {
        <div class="challenge-history">
            <h2 class="challenge-history__title">{ "Challenge History" }</h2>
            <div>
                <h3 class="challenge-history__subtitle">{ "Challenges" }</h3>
                { render_challenge_summary(&props.challenge_history) }
                { render_challenge_table(&props.challenge_history) }
            </div>
        </div>
    }
}

fn render_challenge_summary(challenge_history: &ChallengeHistory) -> Html {
    if challenge_history.challenges.is_empty() {
        html! {
            <p class="challenge-history__text">{ "No challenges completed yet." }</p>
        }
    } else {
        html! {
            <p class="challenge-history__text">
                { format!("{} challenges completed.", challenge_history.challenges.len()) }
            </p>
        }
    }
}

fn render_challenge_table(challenge_history: &ChallengeHistory) -> Html {
    html! {
        <table class="challenge-history__table">
            <thead class="challenge-history__table-head">
                <tr>
                    <th class="challenge-history__table-header">{ "Challenge" }</th>
                    <th class="challenge-history__table-header">{ "Result" }</th>
                </tr>
            </thead>
            <tbody>
                { for challenge_history.challenges.iter().map(render_challenge_row) }
            </tbody>
        </table>
    }
}

fn render_challenge_row(challenge: &Challenge) -> Html {
    html! {
        <tr class="challenge-history__table-row">
            <td class="challenge-history__table-cell">{ &challenge.challenge_type.name() }</td>
            <td class="challenge-history__table-cell">{ &challenge.performance(&challenge.challenge_result) }</td>
        </tr>
    }
}
