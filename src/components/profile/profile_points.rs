use konnektoren_core::prelude::PlayerProfile;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ProfilePointsProps {
    pub profile: PlayerProfile,
}

#[function_component(ProfilePointsComponent)]
pub fn profile_points_component(props: &ProfilePointsProps) -> Html {
    let points = props.profile.xp;

    html! {
        <div class="profile-points">
            <div class="profile-points__icon">{"⭐️"}</div>
            <div class="profile-points__name">{ &props.profile.name }</div>
            <div class="profile-points__points">{ points }</div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ProfilePointsComponent,
        ProfilePointsProps {
            profile: PlayerProfile {
                id: "1".to_string(),
                name: "Test Player".to_string(),
                xp: 100,
            },
        },
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
    );
}
