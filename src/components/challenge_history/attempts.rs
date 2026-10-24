use super::{ChallengeHistoryList, history};
use crate::i18n::use_i18n;
use konnektoren_core::challenges::{Challenge, ChallengeHistory, Performance};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeAttemptHistoryProps {
    pub challenge_history: ChallengeHistory,
    pub challenge_id: AttrValue,
    #[prop_or_default]
    pub limit: Option<usize>,
    #[prop_or_default]
    pub on_select: Option<Callback<Challenge>>,
}

/// All attempts of one configured challenge, with totals and best performance.
#[function_component(ChallengeAttemptHistory)]
pub fn challenge_attempt_history(props: &ChallengeAttemptHistoryProps) -> Html {
    let i18n = use_i18n();
    let entries = history::entries(&props.challenge_history, Some(&props.challenge_id), None);
    let best = entries
        .iter()
        .map(|(_, challenge)| challenge.performance(&challenge.challenge_result))
        .max();
    let name = entries
        .first()
        .map(|(_, challenge)| i18n.t(&history::name(challenge)));
    html! {
        <section class="challenge-history challenge-attempt-history">
            <h2 class="challenge-history__title">{i18n.t("Challenge History")}</h2>
            if let Some(name) = name { <h3 class="challenge-attempt-history__name">{name}</h3> }
            <dl class="challenge-attempt-history__stats">
                <div><dt>{i18n.t("Attempts")}</dt><dd>{entries.len()}</dd></div>
                <div><dt>{i18n.t("Best result")}</dt><dd>{best.map(|best| format!("{best}%")).unwrap_or_else(|| "—".into())}</dd></div>
            </dl>
            <ChallengeHistoryList challenge_history={props.challenge_history.clone()}
                challenge_id={props.challenge_id.clone()} limit={props.limit} on_select={props.on_select.clone()} />
        </section>
    }
}
