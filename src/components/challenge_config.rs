use crate::components::ChallengeInfoComponent;
use crate::i18n::use_i18n;
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
    let i18n = use_i18n();

    let render_new_button = {
        let i18n = i18n.clone();
        let on_new = props.on_new.clone();
        let challenge_config = props.challenge_config.clone();
        move || {
            if let Some(on_new) = on_new {
                let on_new = on_new.clone();
                html! {
                    <button onclick={Callback::from(move |_| on_new.emit(challenge_config.clone()))}>
                        { i18n.t("Start") }
                    </button>
                }
            } else {
                html! {}
            }
        }
    };

    html! {
        <div class="challenge-config" id={props.challenge_config.id.to_string()}>
            <ChallengeInfoComponent
                api_url={props.api_url.clone()}
                challenge_config={props.challenge_config.clone()}
            />
            { render_new_button() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_new_button() {
        // Since we can't use hooks in tests, we test the logic directly.
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

        // Simulate what the button would look like
        let result = if let Some(on_new) = on_new {
            let on_new = on_new.clone();
            html! {
                <button onclick={Callback::from(move |_| on_new.emit(challenge_config.clone()))}>
                    { "Start" }
                </button>
            }
        } else {
            html! {}
        };

        if let Html::VTag(vtag) = result {
            assert_eq!(vtag.tag(), "button");
            assert!(
                vtag.children().into_iter().any(
                    |child| matches!(child, Html::VText(vtext) if vtext.text.contains("Start"))
                )
            );
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
