//! Result view shown after a completed [`Dialog`] challenge.
//!
//! * **Observer mode** — no quiz turns exist; shows a simple completion message.
//! * **Quiz mode**     — shows a recap table with one row per interactive turn,
//!   indicating whether the player's choice was correct.

use crate::i18n::use_i18n;
use konnektoren_core::challenges::{ChallengeResult, Dialog};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DialogResultComponentProps {
    pub challenge: Dialog,
    pub challenge_result: ChallengeResult,
}

#[function_component(DialogResultComponent)]
pub fn dialog_result_component(props: &DialogResultComponentProps) -> Html {
    let i18n = use_i18n();

    let answers = match &props.challenge_result {
        ChallengeResult::Dialog(a) => a.clone(),
        _ => vec![],
    };

    let quiz_turns: Vec<_> = props.challenge.quiz_turns().collect();

    // Observer mode — nothing to grade
    if quiz_turns.is_empty() {
        return html! {
            <div class="dialog-result">
                <p class="dialog-result__observer-msg">
                    { i18n.t("dialog.observer_complete") }
                </p>
            </div>
        };
    }

    let rows = quiz_turns
        .iter()
        .map(|(i, turn)| {
            let answer = answers.iter().find(|a| a.turn_index == *i);

            let (is_correct, chosen_text) = if let Some(ans) = answer {
                let correct = turn.correct_option == Some(ans.selected_option);
                let text = turn
                    .options
                    .as_ref()
                    .and_then(|opts| opts.get(ans.selected_option))
                    .cloned()
                    .unwrap_or_default();
                (correct, text)
            } else {
                (false, i18n.t("dialog.not_answered"))
            };

            let modifier = if is_correct { "correct" } else { "incorrect" };
            let speaker_name = props
                .challenge
                .speaker_by_id(&turn.speaker)
                .map(|s| s.name.as_str())
                .unwrap_or_default()
                .to_owned();

            html! {
                <tr class={classes!(
                    "dialog-result__row",
                    format!("dialog-result__row--{}", modifier)
                )}>
                    <td class="dialog-result__cell dialog-result__cell--speaker">
                        { speaker_name }
                    </td>
                    <td class="dialog-result__cell dialog-result__cell--correct-line">
                        { &turn.text }
                    </td>
                    <td class="dialog-result__cell dialog-result__cell--your-answer">
                        { chosen_text }
                    </td>
                    <td class={classes!(
                        "dialog-result__cell",
                        format!("dialog-result__cell--{}", modifier)
                    )}>
                        { if is_correct { i18n.t("Correct") } else { i18n.t("Incorrect") } }
                    </td>
                </tr>
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div class="dialog-result">
            <h2 class="dialog-result__title">{ i18n.t("Challenge Result") }</h2>
            <div class="dialog-result__table-wrapper">
                <table class="dialog-result__table">
                    <thead>
                        <tr>
                            <th class="dialog-result__header-cell">{ i18n.t("Speaker") }</th>
                            <th class="dialog-result__header-cell">{ i18n.t("Correct answer: ") }</th>
                            <th class="dialog-result__header-cell">{ i18n.t("Your Answer") }</th>
                            <th class="dialog-result__header-cell">{ i18n.t("Result") }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for rows.into_iter() }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::DialogAnswer;
    use yew_preview::prelude::*;

    /// All three quiz turns answered correctly.
    /// Turn 1 correct=0, turn 3 correct=2, turn 5 correct=0.
    fn make_all_correct() -> ChallengeResult {
        ChallengeResult::Dialog(vec![
            DialogAnswer {
                turn_index: 1,
                selected_option: 0,
            },
            DialogAnswer {
                turn_index: 3,
                selected_option: 2,
            },
            DialogAnswer {
                turn_index: 5,
                selected_option: 0,
            },
        ])
    }

    /// Mixed: two wrong picks + one correct.
    ///
    /// Turn 1 options: ["Guten Morgen, Anna! Wie geht's?", "Gute Nacht, Anna!", "Auf Wiedersehen, Anna!"]
    ///   → selected 2 ("Auf Wiedersehen, Anna!") — WRONG, correct is 0
    ///
    /// Turn 3 options: ["Schlecht, danke.", "Ich weiß nicht.", "Auch gut, danke!"]
    ///   → selected 1 ("Ich weiß nicht.") — WRONG, correct is 2
    ///
    /// Turn 5 options: ["Tschüss! Einen schönen Tag noch!", "Guten Morgen!", "Wie geht's?"]
    ///   → selected 0 ("Tschüss! Einen schönen Tag noch!") — CORRECT
    fn make_mixed() -> ChallengeResult {
        ChallengeResult::Dialog(vec![
            DialogAnswer {
                turn_index: 1,
                selected_option: 2,
            },
            DialogAnswer {
                turn_index: 3,
                selected_option: 1,
            },
            DialogAnswer {
                turn_index: 5,
                selected_option: 0,
            },
        ])
    }

    /// Observer mode — no quiz turns, just a completion message.
    fn make_observer_result() -> ChallengeResult {
        ChallengeResult::Dialog(vec![])
    }

    yew_preview::create_preview!(
        DialogResultComponent,
        // Default: all correct
        DialogResultComponentProps {
            challenge: Dialog::default(),
            challenge_result: make_all_correct(),
        },
        (
            "Mixed (incorrect answers)",
            DialogResultComponentProps {
                challenge: Dialog::default(),
                challenge_result: make_mixed(),
            }
        ),
        (
            "Observer mode",
            DialogResultComponentProps {
                challenge: Dialog::default(),
                challenge_result: make_observer_result(),
            }
        ),
    );
}
