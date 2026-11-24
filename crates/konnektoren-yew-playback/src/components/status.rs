use yew::prelude::*;

/// A translated status message for the current playback step.
#[derive(Properties, PartialEq)]
pub struct PlaybackStatusProps {
    pub message: AttrValue,
}

#[function_component(PlaybackStatus)]
pub fn playback_status(props: &PlaybackStatusProps) -> Html {
    html! {
        <div class="playback-status" role="status" aria-live="polite">
            {props.message.clone()}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackStatus,
        PlaybackStatusProps {
            message: "Welcome to the demonstration.".into()
        },
        (
            "Next step",
            PlaybackStatusProps {
                message: "Opening the exercises.".into()
            }
        ),
    );
}
