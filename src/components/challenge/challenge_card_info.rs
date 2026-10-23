use crate::components::ChallengeConfigComponent;
use konnektoren_core::challenges::ChallengeConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeCardInfoProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub api_url: Option<String>,
    /// Emitted when the start button is clicked on an unlocked challenge.
    #[prop_or_default]
    pub on_select: Option<Callback<ChallengeConfig>>,
    /// Emitted instead of `on_select` when the challenge is locked.
    #[prop_or_default]
    pub on_locked: Option<Callback<ChallengeConfig>>,
    #[prop_or(false)]
    pub locked: bool,
}

/// Icon plus config summary shown on the front of a challenge card.
#[function_component(ChallengeCardInfo)]
pub fn challenge_card_info(props: &ChallengeCardInfoProps) -> Html {
    let on_new = if props.locked {
        props.on_locked.clone()
    } else {
        props.on_select.clone()
    };

    let icon = props.challenge_config.icon.as_ref().map(|icon| {
        if icon.starts_with("fa-") {
            html! {
                <div class="challenge-card-info__icon-container">
                    <i class={classes!(icon, "challenge-card-info__icon-fa")}></i>
                </div>
            }
        } else {
            html! {
                <div class="challenge-card-info__icon-container">
                    <img
                        src={icon.clone()}
                        alt={format!("{} icon", props.challenge_config.name)}
                        class="challenge-card-info__icon-image"
                    />
                </div>
            }
        }
    });

    html! {
        <div class="challenge-card-info">
            {icon}
            <ChallengeConfigComponent
                api_url={props.api_url.clone()}
                on_new={on_new}
                challenge_config={props.challenge_config.clone()}
            />
        </div>
    }
}
