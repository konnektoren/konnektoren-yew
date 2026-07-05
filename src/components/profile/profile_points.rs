use crate::i18n::use_i18n;
use crate::model::Design;
use crate::providers::use_design;
use konnektoren_core::prelude::PlayerProfile;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ProfilePointsProps {
    pub profile: PlayerProfile,
}

#[function_component(ProfilePointsComponent)]
pub fn profile_points_component(props: &ProfilePointsProps) -> Html {
    let expanded = use_state(|| false);
    let design = use_design();
    let i18n = use_i18n();
    let points = props.profile.xp;
    let first_letter = props.profile.name.chars().next().unwrap_or('?');
    let is_mobile = matches!(*design, Design::Mobile);

    let toggle_expanded = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    let class = classes!(
        "profile-points",
        is_mobile.then_some("profile-points--mobile"),
        (!is_mobile).then_some("profile-points--desktop"),
        expanded.then(|| "profile-points--expanded")
    );

    if is_mobile {
        html! {
            <div class={class}>
                <button
                    class="profile-points__mobile-trigger"
                    type="button"
                    aria-label={format!("{}: {points}", i18n.t("XP required"))}
                    aria-expanded={expanded.to_string()}
                    onclick={toggle_expanded}
                >
                    <span class="profile-points__avatar">{ first_letter }</span>
                    <span class="profile-points__mobile-score">
                        <i class="fas fa-star profile-points__icon"></i>
                        <span class="profile-points__points">{ points }</span>
                    </span>
                </button>
                <div class="profile-points__mobile-panel">
                    <span class="profile-points__name">{ &props.profile.name }</span>
                    <span class="profile-points__detail">
                        <i class="fas fa-star profile-points__icon"></i>
                        <span>{ points }</span>
                        <span>{ "XP" }</span>
                    </span>
                </div>
            </div>
        }
    } else {
        html! {
            <div class={class}>
                <div class="profile-points__desktop-card">
                    <span class="profile-points__avatar">{ first_letter }</span>
                    <span class="profile-points__identity">
                        <span class="profile-points__name">{ &props.profile.name }</span>
                        <span class="profile-points__caption">{ i18n.t("Player Profile") }</span>
                    </span>
                    <span class="profile-points__desktop-score">
                        <i class="fas fa-star profile-points__icon"></i>
                        <span class="profile-points__points">{ points }</span>
                        <span class="profile-points__caption">{ "XP" }</span>
                    </span>
                </div>
            </div>
        }
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;
    use yew_preview::test_utils::{exists, has_class, has_text};

    yew_preview::create_preview_with_tests!(
        component: ProfilePointsComponent,
        default_props: ProfilePointsProps {
            profile: PlayerProfile {
                id: "1".to_string(),
                name: "Test Player".to_string(),
                xp: 100,
            },
        },
        variants: [
            (
                "long name",
                ProfilePointsProps {
                    profile: PlayerProfile {
                        id: "2".to_string(),
                        name: "Test Player with a long name".to_string(),
                        xp: 0,
                    },
                }
            ),
            (
                "zero xp",
                ProfilePointsProps {
                    profile: PlayerProfile {
                        id: "3".to_string(),
                        name: "Beginner".to_string(),
                        xp: 0,
                    },
                }
            ),
            (
                "high xp",
                ProfilePointsProps {
                    profile: PlayerProfile {
                        id: "4".to_string(),
                        name: "Expert".to_string(),
                        xp: 9999,
                    },
                }
            ),
        ],
        tests: [
            ("Has profile-points wrapper", has_class("profile-points")),
            ("Shows xp points", has_text("100")),
            ("Shows first initial", has_text("T")),
            ("Shows player name", has_text("Test Player")),
            ("Has desktop element", exists("profile-points__desktop-card")),
            ("Has avatar element", exists("profile-points__avatar")),
        ]
    );
}
