use crate::model::PlaybackConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackProviderProps {
    #[prop_or_default]
    pub config: PlaybackConfig,
    pub children: Children,
}

/// Shares presentation defaults with playback components.
#[function_component(PlaybackProvider)]
pub fn playback_provider(props: &PlaybackProviderProps) -> Html {
    html! {
        <ContextProvider<PlaybackConfig> context={props.config.clone()}>
            {props.children.clone()}
        </ContextProvider<PlaybackConfig>>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{PlaybackNotification, PlaybackOverlay};

    #[function_component(ConfiguredPlayback)]
    fn configured_playback() -> Html {
        let config = PlaybackConfig {
            overlay_viewport: Some(true),
            notification_fixed: Some(true),
        };
        html! {
            <PlaybackProvider {config}>
                <PlaybackOverlay step={0}>
                    <PlaybackNotification message="Shared" />
                    <PlaybackNotification message="Override" fixed={Some(false)} />
                </PlaybackOverlay>
            </PlaybackProvider>
        }
    }

    #[tokio::test]
    async fn applies_defaults_and_respects_component_overrides() {
        let html = yew::LocalServerRenderer::<ConfiguredPlayback>::new()
            .render()
            .await;
        assert!(html.contains("playback-overlay--viewport"));
        assert_eq!(html.matches("playback-notification--fixed").count(), 1);
    }
}
