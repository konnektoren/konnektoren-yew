use yew::prelude::*;

use konnektoren_core::achievements::AchievementDefinition;

#[derive(Properties, PartialEq, Default)]
pub struct AchievementComponentProps {
    pub achievement: AchievementDefinition,
}

#[function_component(AchievementComponent)]
pub fn achievement_component(props: &AchievementComponentProps) -> Html {
    let icon_html = if props.achievement.icon.starts_with("fa-") {
        html! {
            <i class={classes!("achievement__icon", "fas", &props.achievement.icon)} aria-hidden="true"></i>
        }
    } else if props.achievement.icon.starts_with("http")
        || props.achievement.icon.starts_with("data:")
    {
        html! {
            <img class="achievement__icon" src={props.achievement.icon.clone()} alt={props.achievement.name.clone()} />
        }
    } else {
        html! {
            <span class="achievement__icon achievement__icon--text">{&props.achievement.icon}</span>
        }
    };

    html! {
        <div class="achievement">
            {icon_html}
            <h3 class="achievement__name">{ &props.achievement.name }</h3>
            <p class="achievement__description">{ &props.achievement.description }</p>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        AchievementComponent,
        AchievementComponentProps {
            achievement: AchievementDefinition {
                name: "Achievement Name".to_string(),
                description: "Achievement Description".to_string(),
                icon: "fa-award".to_string(),
                condition: "".to_string(),
                id: "achievement-id".to_string(),
            },
        },
        (
            "condition not met",
            AchievementComponentProps {
                achievement: AchievementDefinition {
                    name: "Achievement".to_string(),
                    description: "Achievement with unmet condition".to_string(),
                    icon: "fa-praying-hands".to_string(),
                    condition: "false".to_string(),
                    id: "achievement-id".to_string(),
                },
            }
        ),
    );
}
