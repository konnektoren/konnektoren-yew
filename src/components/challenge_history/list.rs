use super::{ChallengeHistoryProps, history};
use crate::i18n::use_i18n;
use konnektoren_core::challenges::Performance;
use yew::prelude::*;

/// A compact, newest-first attempt list for profiles and embedded panels.
#[function_component(ChallengeHistoryList)]
pub fn challenge_history_list(props: &ChallengeHistoryProps) -> Html {
    let i18n = use_i18n();
    if props.limit == Some(0) {
        return Html::default();
    }
    let entries = history::entries(
        &props.challenge_history,
        props.challenge_id.as_deref(),
        props.limit,
    );
    if entries.is_empty() {
        return html! { <p class="challenge-history__empty">{i18n.t("No challenges completed yet.")}</p> };
    }
    html! {
        <ol class="challenge-history-list" aria-label={i18n.t("Challenge History")}>
            {for entries.into_iter().map(|(index, challenge)| {
                let name = i18n.t(&history::name(challenge));
                html! {
                    <li key={index} class="challenge-history-list__item">
                        <div class="challenge-history-list__attempt">
                            if let Some(on_select) = &props.on_select {
                                <button type="button" class="challenge-history__select" onclick={{
                                    let on_select = on_select.clone();
                                    let challenge = challenge.clone();
                                    Callback::from(move |_| on_select.emit(challenge.clone()))
                                }}>{name}</button>
                            } else { <span class="challenge-history-list__name">{name}</span> }
                            <span class="challenge-history-list__date">{history::date(challenge)}</span>
                        </div>
                        <span class="challenge-history-list__score" aria-label={i18n.t("Result")}>
                            {format!("{}%", challenge.performance(&challenge.challenge_result))}
                        </span>
                    </li>
                }
            })}
        </ol>
    }
}
