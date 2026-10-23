use crate::components::challenge::{
    ChallengeCardInfo, ChallengePreviewComponent, ChallengeProgressBadge, best_performance,
};
use konnektoren_core::challenges::{ChallengeConfig, ChallengeHistory};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ChallengeCardProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub challenge_history: ChallengeHistory,
    #[prop_or_default]
    pub api_url: Option<String>,
    #[prop_or(false)]
    pub locked: bool,
    /// Emitted when the start button is clicked on an unlocked challenge.
    #[prop_or_default]
    pub on_select: Option<Callback<ChallengeConfig>>,
    /// Emitted instead of `on_select` when the challenge is locked.
    #[prop_or_default]
    pub on_locked: Option<Callback<ChallengeConfig>>,
    /// Flip the card to show a live preview of the challenge on its back.
    #[prop_or(true)]
    pub flippable: bool,
}

/// Challenge card with progress badge and optional flip-to-preview.
#[function_component(ChallengeCard)]
pub fn challenge_card(props: &ChallengeCardProps) -> Html {
    let flipped = use_state(|| false);

    let on_flip = {
        let flipped = flipped.clone();
        let flippable = props.flippable;
        Callback::from(move |_| {
            if flippable {
                flipped.set(!*flipped);
            }
        })
    };

    let progress = best_performance(&props.challenge_config, &props.challenge_history)
        .map(|percent| html! { <div class="challenge-card__progress"><ChallengeProgressBadge {percent} /></div> });

    let back = props.flippable.then(|| {
        html! {
            <div class="challenge-card__face challenge-card__face--back">
                <div class="challenge-card__preview">
                    if *flipped {
                        <ChallengePreviewComponent challenge_config={props.challenge_config.clone()} />
                    }
                </div>
            </div>
        }
    });

    html! {
        <div class={classes!("challenge-card", props.locked.then_some("challenge-card--locked"))} onclick={on_flip}>
            {progress}
            <div class={classes!("challenge-card__inner", (*flipped).then_some("challenge-card__inner--flipped"))}>
                <div class="challenge-card__face challenge-card__face--front">
                    <ChallengeCardInfo
                        api_url={props.api_url.clone()}
                        challenge_config={props.challenge_config.clone()}
                        locked={props.locked}
                        on_select={props.on_select.clone()}
                        on_locked={props.on_locked.clone()}
                    />
                </div>
                {back}
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::multiple_choice::{MultipleChoice, MultipleChoiceOption};
    use konnektoren_core::prelude::{Challenge, ChallengeResult, ChallengeType, Question};
    use yew_preview::prelude::*;

    const TASKS: usize = 10;

    fn config() -> ChallengeConfig {
        ChallengeConfig {
            id: "preview".to_string(),
            name: "Challenge Name".to_string(),
            description: "Challenge Description".to_string(),
            challenge: "".to_string(),
            variant: None,
            tasks: TASKS.into(),
            unlock_points: 10,
            position: None,
            icon: Some("fa-solid fa-book".to_string()),
        }
    }

    /// History with one finished attempt that got `correct` of `TASKS` answers right.
    fn history(correct: usize) -> ChallengeHistory {
        let challenge_config = config();
        let challenge_type = ChallengeType::MultipleChoice(MultipleChoice {
            id: "preview".to_string(),
            name: "Preview".to_string(),
            lang: "en".to_string(),
            options: vec![],
            questions: (0..TASKS)
                .map(|i| Question {
                    question: format!("Question {i}"),
                    help: String::new(),
                    image: None,
                    option: 0,
                })
                .collect(),
        });
        let answers = (0..TASKS)
            .map(|i| MultipleChoiceOption {
                id: if i < correct { 0 } else { 1 },
                name: String::new(),
            })
            .collect();
        let mut history = ChallengeHistory::new();
        history.add_challenge(Challenge {
            challenge_type,
            challenge_config,
            challenge_result: ChallengeResult::MultipleChoice(answers),
            start_time: None,
            end_time: None,
        });
        history
    }

    fn props(challenge_history: ChallengeHistory, locked: bool) -> ChallengeCardProps {
        ChallengeCardProps {
            challenge_config: config(),
            challenge_history,
            api_url: None,
            locked,
            on_select: (!locked).then(Callback::noop),
            on_locked: locked.then(Callback::noop),
            flippable: false,
        }
    }

    yew_preview::create_preview!(
        ChallengeCard,
        props(ChallengeHistory::default(), false),
        ("30% (no badge)", props(history(3), false)),
        ("50%", props(history(5), false)),
        ("70%", props(history(7), false)),
        ("90%", props(history(9), false)),
        ("100% (solved)", props(history(10), false)),
        ("locked", props(ChallengeHistory::default(), true)),
    );
}
