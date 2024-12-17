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
            <div class="icon">{"⭐️"}</div>
            <div class="profile-name">{ &props.profile.name }</div>
            <div class="points">{ points }</div>
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
    );
}
