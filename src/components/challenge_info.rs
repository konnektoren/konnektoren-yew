use crate::components::{ChallengePresenceComponent, ChallengeRatingComponent};
use crate::i18n::use_i18n;
use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeInfoProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub api_url: Option<String>,
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
        <div class="challenge-info">
            <h2 class="challenge-info__title">{&props.challenge_config.name}</h2>
            {rating_component}
            {presence_component}
            <div class="challenge-info__description">
                <p>{&props.challenge_config.description}</p>
            </div>
            <div class="challenge-info__meta">
                <p class="challenge-info__tasks">{format!("{}: {}", i18n.t("Tasks"), props.challenge_config.tasks.len())}</p>
                <p class="challenge-info__unlock-points">{format!("{}: {}", i18n.t("Unlock Points"), props.challenge_config.unlock_points)}</p>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;
    yew_preview::create_preview!(
        ChallengeInfoComponent,
        ChallengeInfoProps {
            challenge_config: ChallengeConfig {
                id: "".to_string(),
                name: "Challenge Name".to_string(),
                description: "Challenge Description".to_string(),
                challenge: "".to_string(),
                variant: None,
                tasks: 5.into(),
                unlock_points: 10,
                position: None,
            },
            api_url: Some("https://api.example.com".to_string()),
        },
    );
}
