use yew::prelude::*;

/// Displays a playback narration message. The caller translates the message
/// and controls its lifetime as playback advances.
#[derive(Properties, PartialEq)]
pub struct PlaybackNotificationProps {
    pub message: AttrValue,
}

#[function_component(PlaybackNotification)]
pub fn playback_notification(props: &PlaybackNotificationProps) -> Html {
    html! {
        <div class="playback-notification" role="status" aria-live="polite">
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
            message: "Welcome to the demonstration.".into()
        },
        (
            "Long message",
            PlaybackNotificationProps {
                message: "Let's explore the page together and see how to get started.".into()
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
            },
        )
        .render()
        .await;
        assert!(rendered.contains("role=\"status\""));
        assert!(rendered.contains("Look at &lt;this&gt;"));
    }
}
