use crate::model::{NotificationPosition, PlaybackConfig};
use konnektoren_yew_core::ReadText;
use yew::prelude::*;

/// Displays a playback narration message. The caller translates the message
/// and controls its lifetime as playback advances.
#[derive(Properties, PartialEq)]
pub struct PlaybackNotificationProps {
    pub message: AttrValue,
    /// Position the message at the bottom of the viewport.
    #[prop_or_default]
    pub fixed: Option<bool>,
    /// Optional placement, which fixes the notification to the viewport.
    #[prop_or_default]
    pub position: Option<NotificationPosition>,
    /// Speak the displayed, translated message.
    #[prop_or_default]
    pub speak: bool,
    /// Language of the displayed message.
    #[prop_or_else(|| "en-US".to_owned())]
    pub lang: String,
}

#[function_component(PlaybackNotification)]
pub fn playback_notification(props: &PlaybackNotificationProps) -> Html {
    let config = use_context::<PlaybackConfig>().unwrap_or_default();
    let fixed = props.fixed.or(config.notification_fixed).unwrap_or(false);
    let position = props.position.map(|position| match position {
        NotificationPosition::Top => "playback-notification--top",
        NotificationPosition::Bottom => "playback-notification--bottom",
        NotificationPosition::Left => "playback-notification--left",
        NotificationPosition::Right => "playback-notification--right",
    });
    html! {
        <div class={classes!("playback-notification", fixed.then_some("playback-notification--fixed"), position)} role="status" aria-live="polite">
            <p class="playback-notification__message">{props.message.clone()}</p>
            if props.speak && !props.message.trim().is_empty() {
                <ReadText text={props.message.to_string()} lang={props.lang.clone()} />
            }
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
            position: None,
            speak: false,
            lang: "en-US".into(),
        },
        (
            "Long message",
            PlaybackNotificationProps {
                message: "Let's explore the page together and see how to get started.".into(),
                fixed: None,
                position: None,
                speak: false,
                lang: "en-US".into(),
            }
        ),
        (
            "Fixed",
            PlaybackNotificationProps {
                message: "Playback is running.".into(),
                fixed: Some(true),
                position: None,
                speak: false,
                lang: "en-US".into(),
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
                position: None,
                speak: false,
                lang: "en-US".into(),
            },
        )
        .render()
        .await;
        assert!(rendered.contains("role=\"status\""));
        assert!(rendered.contains("Look at &lt;this&gt;"));
    }
}
