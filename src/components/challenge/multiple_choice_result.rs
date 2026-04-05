use crate::i18n::use_i18n;
use konnektoren_core::challenges::{ChallengeResult, MultipleChoice};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct MultipleChoiceResultComponentProps {
    pub challenge: MultipleChoice,
    pub challenge_result: ChallengeResult,
}

#[function_component(MultipleChoiceResultComponent)]
pub fn multiple_choice_result_component(props: &MultipleChoiceResultComponentProps) -> Html {
    let i18n = use_i18n();
    let results = match &props.challenge_result {
        ChallengeResult::MultipleChoice(options) => props
            .challenge
            .questions
            .iter()
            .zip(options.iter())
            .map(|(question, option)| {
                let is_correct = question.option == option.id;
                let modifier = if is_correct { "correct" } else { "incorrect" };
                let correct_name = props
                    .challenge
                    .options
                    .iter()
                    .find(|o| o.id == question.option)
                    .map(|o| o.name.clone())
                    .unwrap_or_default();

                html! {
                    <tr class={classes!("multiple-choice-result__row", format!("multiple-choice-result__row--{}", modifier))}>
                        <td class="multiple-choice-result__cell">
                            { &question.question }
                        </td>
                        <td class={classes!("multiple-choice-result__cell", format!("multiple-choice-result__cell--{}", modifier))}>
                            if is_correct {
                                { format!("✅ {}", option.name) }
                            } else {
                                <>
                                    { format!("✘ {} → ✅ ", option.name) }
                                    <span class="multiple-choice-result__correct-answer">{ correct_name }</span>
                                </>
                            }
                        </td>
                    </tr>
                }
            })
            .collect::<Vec<Html>>(),
        _ => panic!("Invalid challenge type"),
    };

    html! {
        <div class="multiple-choice-result">
            <h2 class="multiple-choice-result__title">{ i18n.t("Challenge Result") }</h2>
            <table class="multiple-choice-result__table">
                <thead class="multiple-choice-result__header">
                    <tr>
                        <th class="multiple-choice-result__header-cell">{ i18n.t("Question") }</th>
                        <th class="multiple-choice-result__header-cell">{ i18n.t("Result") }</th>
                    </tr>
                </thead>
                <tbody class="multiple-choice-result__body">
                    {for results.into_iter()}
                </tbody>
            </table>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::MultipleChoiceOption;
    use konnektoren_core::prelude::{ChallengeType, Game};
    use yew_preview::prelude::*;
    use yew_preview::test_utils::{exists, has_class, has_text};

    fn articles_challenge() -> MultipleChoice {
        let game = Game::default();
        let challenge = game.create_challenge("articles-1").unwrap();
        match challenge.challenge_type {
            ChallengeType::MultipleChoice(mc) => mc,
            _ => unreachable!(),
        }
    }

    /// Build a result where the first `correct` questions are answered right
    /// and the rest are answered with the first available (wrong) option.
    fn make_result(challenge: &MultipleChoice, correct: usize) -> ChallengeResult {
        let options = challenge
            .questions
            .iter()
            .enumerate()
            .map(|(i, q)| {
                if i < correct {
                    challenge
                        .options
                        .iter()
                        .find(|o| o.id == q.option)
                        .cloned()
                        .unwrap_or_else(|| challenge.options[0].clone())
                } else {
                    challenge
                        .options
                        .iter()
                        .find(|o| o.id != q.option)
                        .cloned()
                        .unwrap_or_else(|| MultipleChoiceOption { id: 0, name: "?".into() })
                }
            })
            .collect();
        ChallengeResult::MultipleChoice(options)
    }

    yew_preview::create_preview_with_tests!(
        component: MultipleChoiceResultComponent,
        default_props: MultipleChoiceResultComponentProps {
            challenge: articles_challenge(),
            challenge_result: {
                let ch = articles_challenge();
                make_result(&ch, ch.questions.len())
            },
        },
        variants: [
            (
                "Mixed results",
                MultipleChoiceResultComponentProps {
                    challenge: articles_challenge(),
                    challenge_result: {
                        let ch = articles_challenge();
                        make_result(&ch, 3)
                    },
                }
            ),
            (
                "All incorrect",
                MultipleChoiceResultComponentProps {
                    challenge: articles_challenge(),
                    challenge_result: {
                        let ch = articles_challenge();
                        make_result(&ch, 0)
                    },
                }
            ),
        ],
        tests: [
            ("Has result table", exists("table")),
            ("Has result rows", exists("tr")),
            ("Has correct CSS class", has_class("multiple-choice-result")),
            ("Shows checkmark for correct answers", has_text("✓")),
            ("Shows correct answer hint for incorrect rows", has_text("→")),
            ("Correct answer hint uses correct CSS class", has_class("multiple-choice-result__correct-answer")),
        ]
    );
}
