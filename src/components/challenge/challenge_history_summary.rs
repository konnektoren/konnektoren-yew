use crate::i18n::use_i18n;
use konnektoren_core::challenges::ChallengeHistory;
use konnektoren_core::prelude::{Challenge, Performance};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeHistorySummaryProps {
    pub challenge_history: ChallengeHistory,
}

#[function_component(ChallengeHistorySummaryComponent)]
pub fn challenge_history_summary(props: &ChallengeHistorySummaryProps) -> Html {
    let i18n = use_i18n();
    html! {
        <div class="challenge-history">
            <h2 class="challenge-history__title">
                { i18n.t("Challenge History") }
            </h2>
            <div>
                <h3 class="challenge-history__subtitle">
                    { i18n.t("Challenges") }
                </h3>
                <ChallengeSummary challenge_history={props.challenge_history.clone()} />
                <ChallengeTable challenge_history={props.challenge_history.clone()} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChallengeSummaryProps {
    pub challenge_history: ChallengeHistory,
}

#[function_component(ChallengeSummary)]
fn challenge_summary(props: &ChallengeSummaryProps) -> Html {
    let i18n = use_i18n();
    if props.challenge_history.challenges.is_empty() {
        html! {
            <p class="challenge-history__text">{ i18n.t("No challenges completed yet.") }</p>
        }
    } else {
        html! {
            <p class="challenge-history__text">
                { format!("{} {}", props.challenge_history.challenges.len(), i18n.t("challenges completed.")) }
            </p>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ChallengeTableProps {
    pub challenge_history: ChallengeHistory,
}

#[function_component(ChallengeTable)]
fn challenge_table(props: &ChallengeTableProps) -> Html {
    let i18n = use_i18n();
    html! {
        <table class="challenge-history__table">
            <thead class="challenge-history__table-head">
                <tr>
                    <th class="challenge-history__table-header">{ i18n.t("Challenge") }</th>
                    <th class="challenge-history__table-header">{ i18n.t("Result") }</th>
                </tr>
            </thead>
            <tbody>
                { for props.challenge_history.challenges.iter().map(|c| html!{ <ChallengeRow challenge={c.clone()} /> }) }
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChallengeRowProps {
    pub challenge: Challenge,
}

#[function_component(ChallengeRow)]
fn challenge_row(props: &ChallengeRowProps) -> Html {
    let i18n = use_i18n();
    html! {
        <tr class="challenge-history__table-row">
            <td class="challenge-history__table-cell">{ i18n.t(&props.challenge.challenge_type.name()) }</td>
            <td class="challenge-history__table-cell">{ props.challenge.performance(&props.challenge.challenge_result) }</td>
        </tr>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::ChallengeHistory;
    use konnektoren_core::prelude::{Challenge, ChallengeResult, ChallengeType, Question};
    use yew_preview::prelude::*;

    fn sample_history() -> ChallengeHistory {
        let mut history = ChallengeHistory::new();
        // Add a sample challenge
        let challenge = Challenge {
            challenge_type: ChallengeType::MultipleChoice(
                konnektoren_core::challenges::multiple_choice::MultipleChoice {
                    id: "mc-1".to_string(),
                    name: "Sample Multiple Choice".to_string(),
                    lang: "en".to_string(),
                    options: vec![],
                    questions: vec![Question {
                        question: "What is the capital of Germany?".to_string(),
                        help: "Berlin".to_string(),
                        image: None,
                        option: 0,
                    }],
                },
            ),
            challenge_config: Default::default(),
            challenge_result: ChallengeResult::default(),
            start_time: None,
            end_time: None,
        };
        history.add_challenge(challenge);
        history
    }

    yew_preview::create_preview!(
        ChallengeHistorySummaryComponent,
        ChallengeHistorySummaryProps {
            challenge_history: sample_history(),
        },
        (
            "empty",
            ChallengeHistorySummaryProps {
                challenge_history: ChallengeHistory::new(),
            }
        )
    );
}
