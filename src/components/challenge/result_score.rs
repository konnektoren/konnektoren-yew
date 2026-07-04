use crate::i18n::use_i18n;
use konnektoren_core::challenges::{
    Challenge, ChallengeResult, ChallengeType, ChallengeVariant, Performance,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultScoreComponentProps {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
}

fn is_unscored(challenge: &Challenge) -> bool {
    matches!(challenge.challenge_type, ChallengeType::Informative(_))
        || matches!(
            challenge.challenge_config.variant,
            Some(ChallengeVariant::DialogObserver)
                | Some(ChallengeVariant::InformativeText)
                | Some(ChallengeVariant::InformativeMarkdown)
        )
}

#[function_component(ResultScoreComponent)]
pub fn result_score_component(props: &ResultScoreComponentProps) -> Html {
    let i18n = use_i18n();

    if is_unscored(&props.challenge) {
        return html! {
            <div class="result-score result-score--completed">
                <span class="result-score__label">{ i18n.t("Completed!") }</span>
            </div>
        };
    }

    let performance = props.challenge.performance(&props.challenge_result);
    let modifier = match performance {
        p if p >= 90 => "excellent",
        p if p >= 70 => "good",
        p if p >= 50 => "fair",
        _ => "needs-improvement",
    };
    let label = match performance {
        p if p >= 90 => i18n.t("Excellent!"),
        p if p >= 70 => i18n.t("Good job!"),
        p if p >= 50 => i18n.t("Fair attempt."),
        _ => i18n.t("Keep practicing!"),
    };

    html! {
        <div class={classes!("result-score", format!("result-score--{}", modifier))}>
            <span class="result-score__percentage">{ format!("{}%", performance) }</span>
            <span class="result-score__label">{ label }</span>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::Dialog;
    use konnektoren_core::prelude::{ChallengeConfig, ChallengeType};
    use yew_preview::prelude::*;

    fn scored_challenge(performance: u32) -> (Challenge, ChallengeResult) {
        use konnektoren_core::challenges::{MultipleChoice, MultipleChoiceOption, Question};
        let n = 10u32;
        let correct = performance / 10;
        let questions = (0..n)
            .map(|i| Question {
                question: format!("Q{}", i),
                help: String::new(),
                image: None,
                option: if i < correct { 1 } else { 0 },
            })
            .collect::<Vec<_>>();
        let options = vec![
            MultipleChoiceOption {
                id: 0,
                name: "Wrong".to_string(),
            },
            MultipleChoiceOption {
                id: 1,
                name: "Correct".to_string(),
            },
        ];
        let answers = questions
            .iter()
            .map(|q| MultipleChoiceOption {
                id: q.option,
                name: String::new(),
            })
            .collect();
        let challenge = Challenge {
            challenge_type: ChallengeType::MultipleChoice(MultipleChoice {
                id: "test".to_string(),
                name: "Test".to_string(),
                lang: "en".to_string(),
                questions,
                options,
            }),
            challenge_config: ChallengeConfig::default(),
            challenge_result: ChallengeResult::default(),
            start_time: None,
            end_time: None,
        };
        (challenge, ChallengeResult::MultipleChoice(answers))
    }

    fn observer_challenge() -> (Challenge, ChallengeResult) {
        let challenge = Challenge {
            challenge_type: ChallengeType::Dialog(Dialog::default()),
            challenge_config: ChallengeConfig {
                variant: Some(ChallengeVariant::DialogObserver),
                ..ChallengeConfig::default()
            },
            challenge_result: ChallengeResult::Dialog(vec![]),
            start_time: None,
            end_time: None,
        };
        (challenge, ChallengeResult::Dialog(vec![]))
    }

    yew_preview::create_preview!(
        ResultScoreComponent,
        {
            let (challenge, challenge_result) = scored_challenge(100);
            ResultScoreComponentProps {
                challenge,
                challenge_result,
            }
        },
        ("Good (70%)", {
            let (challenge, challenge_result) = scored_challenge(70);
            ResultScoreComponentProps {
                challenge,
                challenge_result,
            }
        }),
        ("Fair (50%)", {
            let (challenge, challenge_result) = scored_challenge(50);
            ResultScoreComponentProps {
                challenge,
                challenge_result,
            }
        }),
        ("Needs improvement (30%)", {
            let (challenge, challenge_result) = scored_challenge(30);
            ResultScoreComponentProps {
                challenge,
                challenge_result,
            }
        }),
        ("Observer dialog (unscored)", {
            let (challenge, challenge_result) = observer_challenge();
            ResultScoreComponentProps {
                challenge,
                challenge_result,
            }
        }),
    );
}
