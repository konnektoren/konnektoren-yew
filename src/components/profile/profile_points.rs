use konnektoren_core::prelude::PlayerProfile;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ProfilePointsProps {
    pub profile: PlayerProfile,
}

#[function_component(ProfilePointsComponent)]
pub fn profile_points_component(props: &ProfilePointsProps) -> Html {
    let expanded = use_state(|| false);
    let points = props.profile.xp;
    let first_letter = props.profile.name.chars().next().unwrap_or('?');

    let toggle_expanded = {
        let expanded = expanded.clone();
        Callback::from(move |e: MouseEvent| {
            #[cfg(feature = "csr")]
            {
                e.prevent_default();
                expanded.set(!*expanded);
            }
        })
    };

    let class = classes!(
        "profile-points",
        expanded.then(|| "profile-points--expanded")
    );

    html! {
        <div {class} onclick={toggle_expanded}>
            <div class="profile-points__badge">
                <div class="profile-points__badge-top">
                    <span class="profile-points__icon">{"⭐️"}</span>
                    <span class="profile-points__initial">{ first_letter }</span>
                </div>
                <div class="profile-points__points">{ points }</div>
            </div>
            <div class="profile-points__expanded">
                <span class="profile-points__icon">{"⭐️"}</span>
                <span class="profile-points__points">{ points }</span>
                <span class="profile-points__name">{ &props.profile.name }</span>
            </div>
        </div>
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
            ("Shows star icon", has_text("⭐️")),
            ("Shows xp points", has_text("100")),
            ("Shows first initial", has_text("T")),
            ("Shows player name", has_text("Test Player")),
            ("Has badge element", exists("profile-points__badge")),
            ("Has expanded element", exists("profile-points__expanded")),
        ]
    );
}
