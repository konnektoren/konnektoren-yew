use crate::components::ChallengeInfoComponent;
use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeConfigComponentProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub on_new: Option<Callback<ChallengeConfig>>,
    #[prop_or_default]
    pub api_url: Option<String>,
}

#[function_component(ChallengeConfigComponent)]
pub fn challenge_config_component(props: &ChallengeConfigComponentProps) -> Html {
    html! {
        <div class="challenge-config" id={props.challenge_config.id.to_string()}>
            <ChallengeInfoComponent api_url={props.api_url.clone()} challenge_config={props.challenge_config.clone()} />
            {render_new_button(&props.on_new, props.challenge_config.clone())}
        </div>
    }
}

fn render_new_button(
    on_new: &Option<Callback<ChallengeConfig>>,
    challenge_config: ChallengeConfig,
) -> Html {
    if let Some(on_new) = &on_new {
        let on_new = on_new.clone();
        html! {
            <button onclick={Callback::from(move |_| on_new.emit(challenge_config.clone()))}>
                {"Start"}
            </button>
        }
    } else {
        html! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_new_button() {
        let challenge_config = ChallengeConfig {
            id: "konnektoren".to_string(),
            name: "Konnektoren".to_string(),
            description: "Konnektoren".to_string(),
            tasks: 2.into(),
            unlock_points: 10,
            challenge: "konnektoren".to_string(),
            variant: None,
            position: None,
        };
        let on_new = Some(Callback::noop());
        let result = render_new_button(&on_new, challenge_config.clone());

        if let Html::VTag(vtag) = result {
            assert_eq!(vtag.tag(), "button");
            assert!(vtag
                .children()
                .into_iter()
                .any(|child| matches!(child, Html::VText(vtext) if vtext.text.contains("Start"))));
        } else {
            panic!("Expected VTag");
        }
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeConfigComponent,
        ChallengeConfigComponentProps {
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
            on_new: None,
            api_url: None,
        },
    );
}
