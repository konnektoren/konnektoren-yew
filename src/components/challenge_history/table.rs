use super::{ChallengeHistoryProps, history};
use crate::i18n::use_i18n;
use konnektoren_core::challenges::Performance;
use yew::prelude::*;

/// Tabular history with a date and score for every recorded attempt.
#[function_component(ChallengeHistoryTable)]
pub fn challenge_history_table(props: &ChallengeHistoryProps) -> Html {
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
        <div class="challenge-history-table">
            <table class="challenge-history-table__table">
                <caption class="sr-only">{i18n.t("Challenge History")}</caption>
                <thead><tr>
                    <th scope="col">{i18n.t("Challenge")}</th>
                    <th scope="col">{i18n.t("Date")}</th>
                    <th scope="col">{i18n.t("Result")}</th>
                </tr></thead>
                <tbody>
                    {for entries.into_iter().map(|(index, challenge)| html! {
                        <tr key={index}>
                            <td>
                                if let Some(on_select) = &props.on_select {
                                    <button type="button" class="challenge-history__select" onclick={{
                                        let on_select = on_select.clone();
                                        let challenge = challenge.clone();
                                        Callback::from(move |_| on_select.emit(challenge.clone()))
                                    }}>{i18n.t(&history::name(challenge))}</button>
                                } else { {i18n.t(&history::name(challenge))} }
                            </td>
                            <td>{history::date(challenge)}</td>
                            <td>{format!("{}%", challenge.performance(&challenge.challenge_result))}</td>
                        </tr>
                    })}
                </tbody>
            </table>
        </div>
    }
}
