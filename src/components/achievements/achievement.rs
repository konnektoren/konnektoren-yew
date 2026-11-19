use crate::components::Icon;
use crate::i18n::use_i18n;
use konnektoren_core::achievements::AchievementDefinition;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct AchievementComponentProps {
    pub achievement: AchievementDefinition,
}

#[function_component(AchievementComponent)]
pub fn achievement_component(props: &AchievementComponentProps) -> Html {
    let i18n = use_i18n();

    html! {
        <div class="achievement">
            <Icon icon={props.achievement.icon.clone()} alt={i18n.t(&props.achievement.name)} class="achievement__icon" />
            <h3 class="achievement__name">{ i18n.t(&props.achievement.name) }</h3>
            <p class="achievement__description">{ i18n.t(&props.achievement.description) }</p>
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
