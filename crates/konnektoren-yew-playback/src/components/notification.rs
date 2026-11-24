use crate::model::PlaybackConfig;
use yew::prelude::*;

/// Displays a playback narration message. The caller translates the message
/// and controls its lifetime as playback advances.
#[derive(Properties, PartialEq)]
pub struct PlaybackNotificationProps {
    pub message: AttrValue,
    /// Position the message at the bottom of the viewport.
    #[prop_or_default]
    pub fixed: Option<bool>,
}

#[function_component(PlaybackNotification)]
pub fn playback_notification(props: &PlaybackNotificationProps) -> Html {
    let config = use_context::<PlaybackConfig>().unwrap_or_default();
    let fixed = props.fixed.or(config.notification_fixed).unwrap_or(false);
    html! {
        <div class={classes!("playback-notification", fixed.then_some("playback-notification--fixed"))} role="status" aria-live="polite">
            <p class="playback-notification__message">{props.message.clone()}</p>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackNotification,
        PlaybackNotificationProps {
            message: "Welcome to the demonstration.".into(),
            fixed: None,
        },
        (
            "Long message",
            PlaybackNotificationProps {
                message: "Let's explore the page together and see how to get started.".into(),
                fixed: None,
            }
        ),
        (
            "Fixed",
            PlaybackNotificationProps {
                message: "Playback is running.".into(),
                fixed: Some(true)
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn announces_and_escapes_the_message() {
        let rendered = yew::LocalServerRenderer::<PlaybackNotification>::with_props(
            PlaybackNotificationProps {
                message: "Look at <this>".into(),
                fixed: None,
            },
        )
        .render()
        .await;
        assert!(rendered.contains("role=\"status\""));
        assert!(rendered.contains("Look at &lt;this&gt;"));
    }
}
