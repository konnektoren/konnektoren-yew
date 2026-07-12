mod achievement;
#[cfg(feature = "certificates")]
mod achievements;
mod achievement_unlocked;

pub use achievement::AchievementComponent;
#[cfg(feature = "certificates")]
pub use achievements::AchievementsComponent;
pub use achievement_unlocked::AchievementUnlockedComponent;

use yew::prelude::*;

/// Renders an achievement icon: `fa-*` -> Font Awesome, `http`/`data:` -> image, else raw text/emoji.
pub(super) fn render_icon(icon: &str, alt: &str, base_class: &str) -> Html {
    if icon.starts_with("fa-") {
        html! {
            <i class={classes!(base_class.to_string(), "fas", icon.to_string())} aria-hidden="true"></i>
        }
    } else if icon.starts_with("http") || icon.starts_with("data:") {
        html! {
            <img class={base_class.to_string()} src={icon.to_string()} alt={alt.to_string()} />
        }
    } else {
        html! {
            <span class={classes!(base_class.to_string(), format!("{base_class}--text"))}>{icon.to_string()}</span>
        }
    }
}
