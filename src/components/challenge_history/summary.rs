use super::{ChallengeHistoryProps, ChallengeHistoryTable, history};
use crate::i18n::use_i18n;
use yew::prelude::*;

pub type ChallengeHistorySummaryProps = ChallengeHistoryProps;

/// A complete history panel; use the list or table directly inside an existing section.
#[function_component(ChallengeHistorySummaryComponent)]
pub fn challenge_history_summary(props: &ChallengeHistorySummaryProps) -> Html {
    let i18n = use_i18n();
    let count = history::entries(
        &props.challenge_history,
        props.challenge_id.as_deref(),
        None,
    )
    .len();
    html! {
        <section class="challenge-history">
            <h2 class="challenge-history__title">{i18n.t("Challenge History")}</h2>
            if count > 0 {
                <p class="challenge-history__count">{format!("{}: {count}", i18n.t("Attempts"))}</p>
            }
            <ChallengeHistoryTable ..props.clone() />
        </section>
    }
}
