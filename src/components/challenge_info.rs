use crate::components::{ChallengePresenceComponent, ChallengeRatingComponent};
use crate::i18n::use_i18n;
use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeInfoProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub api_url: Option<String>,
    #[prop_or(true)]
    pub open: bool,
}

#[function_component(ChallengeInfoComponent)]
pub fn challenge_info(props: &ChallengeInfoProps) -> Html {
    let i18n = use_i18n();

    let rating_component = match props.api_url {
        Some(ref api_url) => html! {
            <div class="challenge-info__rating">
                <ChallengeRatingComponent api_url={api_url.clone()} challenge_id={props.challenge_config.id.clone()} />
            </div>
        },
        None => html! {},
    };

    let presence_component = match props.api_url {
        Some(ref api_url) => html! {
        <div class="challenge-info__presence">
            <ChallengePresenceComponent api_url={api_url.clone()} challenge_id={props.challenge_config.id.clone()} read_only={true} />
        </div>
        },
        None => html! {},
    };

    html! {
        <details class="challenge-info" open={props.open}>
            <summary class="challenge-info__title">
                <span class="challenge-info__title-text">{ i18n.t(&props.challenge_config.name) }</span>
                <span class="challenge-info__summary-meta">
                    <span
                        class="challenge-info__summary-pill challenge-info__summary-pill--tasks tooltip"
                        data-tip={i18n.t("Number of exercises")}
                    >
                        <i class="fas fa-list-check"></i>
                        { props.challenge_config.tasks.len() }
                    </span>
                    if props.challenge_config.unlock_points > 0 {
                        <span
                            class="challenge-info__summary-pill challenge-info__summary-pill--xp tooltip"
                            data-tip={i18n.t("XP needed to unlock")}
                        >
                            <i class="fas fa-lock"></i>
                            { props.challenge_config.unlock_points }
                        </span>
                    } else {
                        <span
                            class="challenge-info__summary-pill challenge-info__summary-pill--free tooltip"
                            data-tip={i18n.t("Free to play")}
                        >
                            <i class="fas fa-unlock"></i>
                        </span>
                    }
                </span>
            </summary>

            <div class="challenge-info__content">
                if props.api_url.is_some() {
                    <div class="challenge-info__signals">
                        {rating_component}
                        {presence_component}
                    </div>
                }

                <div class="challenge-info__description">
                    <p>{ i18n.t(&props.challenge_config.description) }</p>
                </div>
            </div>
        </details>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;
    yew_preview::create_preview!(
        ChallengeInfoComponent,
        // default: open, with api, locked
        ChallengeInfoProps {
            challenge_config: ChallengeConfig {
                id: "challenge-1".to_string(),
                name: "Konjunktiv II".to_string(),
                description: "Übe den Konjunktiv II in Wunsch- und Bedingungssätzen.".to_string(),
                challenge: "multiple-choice".to_string(),
                variant: None,
                tasks: 5.into(),
                unlock_points: 10,
                position: None,
                icon: None
            },
            api_url: Some("https://api.example.com".to_string()),
            open: true,
        },
        (
            "collapsed",
            ChallengeInfoProps {
                challenge_config: ChallengeConfig {
                    id: "challenge-1".to_string(),
                    name: "Konjunktiv II".to_string(),
                    description: "Übe den Konjunktiv II in Wunsch- und Bedingungssätzen.".to_string(),
                    challenge: "multiple-choice".to_string(),
                    variant: None,
                    tasks: 5.into(),
                    unlock_points: 10,
                    position: None,
                    icon: None
                },
                api_url: Some("https://api.example.com".to_string()),
                open: false,
            }
        ),
        (
            "free (0 XP)",
            ChallengeInfoProps {
                challenge_config: ChallengeConfig {
                    id: "challenge-free".to_string(),
                    name: "Artikel: der, die, das".to_string(),
                    description: "Lerne die Artikel für häufige Nomen.".to_string(),
                    challenge: "multiple-choice".to_string(),
                    variant: None,
                    tasks: 10.into(),
                    unlock_points: 0,
                    position: None,
                    icon: None
                },
                api_url: None,
                open: true,
            }
        ),
        (
            "many tasks, high XP",
            ChallengeInfoProps {
                challenge_config: ChallengeConfig {
                    id: "challenge-hard".to_string(),
                    name: "Erweiterter Gebrauch des Konjunktivs".to_string(),
                    description: "Vertiefe deine Kenntnisse der deutschen Konjunktiv-Modi für formelle und wissenschaftliche Kontexte.".to_string(),
                    challenge: "multiple-choice".to_string(),
                    variant: None,
                    tasks: 20.into(),
                    unlock_points: 50,
                    position: None,
                    icon: None
                },
                api_url: Some("https://api.example.com".to_string()),
                open: true,
            }
        ),
        (
            "no api_url",
            ChallengeInfoProps {
                challenge_config: ChallengeConfig {
                    id: "challenge-offline".to_string(),
                    name: "Präpositionen mit Akkusativ".to_string(),
                    description: "Übe Präpositionen, die den Akkusativ verlangen.".to_string(),
                    challenge: "multiple-choice".to_string(),
                    variant: None,
                    tasks: 8.into(),
                    unlock_points: 5,
                    position: None,
                    icon: None
                },
                api_url: None,
                open: true,
            }
        ),
    );
}
