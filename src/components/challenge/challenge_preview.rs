use crate::components::challenge::ChallengeEffectComponent;
use crate::providers::use_game_state;
use konnektoren_core::challenges::Timed;
use konnektoren_core::prelude::ChallengeConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengePreviewProps {
    pub challenge_config: ChallengeConfig,
    /// Presence API shown (read-only) on the preview. No presence if `None`.
    #[prop_or_default]
    pub api_url: Option<AttrValue>,
}

/// Read-only preview of a challenge, e.g. for the back of a challenge card.
#[function_component(ChallengePreviewComponent)]
pub fn challenge_preview_component(props: &ChallengePreviewProps) -> Html {
    let game_state = use_game_state();
    let game = game_state.game.clone();
    let challenge_config = props.challenge_config.clone();

    let challenge = use_state(|| {
        game.create_challenge(&challenge_config.id)
            .ok()
            .map(|mut challenge| {
                challenge.start();
                challenge
            })
    });

    match *challenge {
        Some(ref challenge) => html! {
            <div class="challenge-preview">
                <ChallengeEffectComponent
                    challenge={challenge.clone()}
                    variant={challenge_config.variant.clone()}
                    preview={true}
                    api_url={props.api_url.clone()}
                />
            </div>
        },
        None => html! {
            <div class="challenge-preview">
                <p>{"Loading"}</p>
            </div>
        },
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::prelude::Game;
    use yew_preview::prelude::*;

    fn props(id: &str) -> ChallengePreviewProps {
        ChallengePreviewProps {
            challenge_config: Game::default()
                .create_challenge(id)
                .unwrap()
                .challenge_config,
            api_url: None,
        }
    }

    yew_preview::create_preview!(
        ChallengePreviewComponent,
        props("konnektoren-1"),
        ("Articles", props("articles-1")),
        ("Past tense", props("past-tense-1")),
        (
            "Sentence structure (ordering)",
            props("sentence-structure-1")
        ),
        ("Dialog observer", props("dialog-begruessung-observer")),
        ("Dialog quiz", props("dialog-begruessung-quiz")),
        (
            "unknown challenge",
            ChallengePreviewProps {
                challenge_config: ChallengeConfig::default(),
                api_url: None,
            }
        ),
    );
}
