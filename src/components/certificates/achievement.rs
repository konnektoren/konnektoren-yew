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
