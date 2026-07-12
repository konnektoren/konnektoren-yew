use super::render_icon;
use crate::i18n::use_i18n;
use konnektoren_core::achievements::AchievementDefinition;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct AchievementUnlockedComponentProps {
    pub achievement: AchievementDefinition,
    /// Renders already opened, skipping the locked/tap-to-reveal state.
    /// Defaults to locked, so a chest is shown that the user taps to reveal the achievement.
    #[prop_or(false)]
    pub open: bool,
}

/// A locked chest that the user taps to reveal a newly-earned achievement — pops
/// open, icon bursts out. Pair with [`super::AchievementComponent`] for the
/// plain, non-celebratory card used in achievement lists.
#[function_component(AchievementUnlockedComponent)]
pub fn achievement_unlocked_component(props: &AchievementUnlockedComponentProps) -> Html {
    let i18n = use_i18n();
    let achievement = &props.achievement;
    let opened = use_state(|| props.open);

    let onclick = {
        let opened = opened.clone();
        Callback::from(move |_| opened.set(true))
    };

    let container_class = classes!(
        "achievement-unlocked",
        opened.then_some("achievement-unlocked--open"),
    );

    html! {
        <div class={container_class}>
            <button
                type="button"
                class="achievement-unlocked__chest"
                onclick={onclick}
                disabled={*opened}
                aria-label={i18n.t("Tap to reveal achievement")}
            >
                <i class="achievement-unlocked__chest-icon fa-solid fa-gift" aria-hidden="true"></i>
                if !*opened {
                    <i class="achievement-unlocked__lock fa-solid fa-lock" aria-hidden="true"></i>
                }
                if *opened {
                    <span class="achievement-unlocked__burst"></span>
                    <span class="achievement-unlocked__sparkle achievement-unlocked__sparkle--1"></span>
                    <span class="achievement-unlocked__sparkle achievement-unlocked__sparkle--2"></span>
                    <span class="achievement-unlocked__sparkle achievement-unlocked__sparkle--3"></span>
                    { render_icon(&achievement.icon, &i18n.t(&achievement.name), "achievement-unlocked__icon") }
                }
            </button>
            <div class="achievement-unlocked__content">
                <span class="achievement-unlocked__label">
                    { if *opened { i18n.t("New Achievement Unlocked!") } else { i18n.t("New Achievement!") } }
                </span>
                <h3 class="achievement-unlocked__name">
                    { if *opened { i18n.t(&achievement.name) } else { "???".to_string() } }
                </h3>
                <p class="achievement-unlocked__description">
                    { if *opened { i18n.t(&achievement.description) } else { i18n.t("Tap the chest to reveal your reward") } }
                </p>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        AchievementUnlockedComponent,
        AchievementUnlockedComponentProps {
            achievement: AchievementDefinition {
                name: "First Exercise".to_string(),
                description: "Complete your first exercise".to_string(),
                icon: "fa-award".to_string(),
                condition: "".to_string(),
                id: "achievement-id".to_string(),
            },
            open: false,
        },
        (
            "already open",
            AchievementUnlockedComponentProps {
                achievement: AchievementDefinition {
                    name: "XP Master".to_string(),
                    description: "Earn 1000 XP".to_string(),
                    icon: "🏆".to_string(),
                    condition: "".to_string(),
                    id: "achievement-id".to_string(),
                },
                open: true,
            }
        ),
        (
            "locked, image icon",
            AchievementUnlockedComponentProps {
                achievement: AchievementDefinition {
                    name: "Path Explorer".to_string(),
                    description: "Complete 3 game paths".to_string(),
                    icon: "https://picsum.photos/64".to_string(),
                    condition: "".to_string(),
                    id: "achievement-id".to_string(),
                },
                open: false,
            }
        ),
    );
}
