use crate::model::PlaybackConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackOverlayProps {
    /// Reopen the overlay when the step changes, after a page may open a dialog.
    pub step: usize,
    /// Cover the viewport and use the browser top layer for modal dialogs.
    #[prop_or_default]
    pub viewport: Option<bool>,
    pub children: Children,
}

/// Places playback guidance above native modal dialogs.
#[function_component(PlaybackOverlay)]
pub fn playback_overlay(props: &PlaybackOverlayProps) -> Html {
    let node = use_node_ref();
    let config = use_context::<PlaybackConfig>().unwrap_or_default();
    let viewport = props.viewport.or(config.overlay_viewport).unwrap_or(false);
    #[cfg(feature = "csr")]
    {
        let node = node.clone();
        use_effect_with((props.step, viewport), move |(_, viewport)| {
            let timer = viewport.then(|| {
                gloo_timers::callback::Timeout::new(0, move || {
                    if let Some(element) = node.cast::<web_sys::HtmlElement>() {
                        let _ = element.hide_popover();
                        let _ = element.show_popover();
                    }
                })
            });
            move || drop(timer)
        });
    }
    html! {
        <div ref={node} class={classes!("playback-overlay", viewport.then_some("playback-overlay--viewport"))} popover={if cfg!(feature = "csr") && viewport { Some("manual") } else { None }}>
            {props.children.clone()}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::components::PlaybackNotification;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackOverlay,
        PlaybackOverlayProps {
            step: 0,
            viewport: None,
            children: Children::new(vec![
                html! { <PlaybackNotification message="Playback guidance stays above dialogs." /> }
            ]),
        },
        (
            "Viewport",
            PlaybackOverlayProps {
                step: 0,
                viewport: Some(true),
                children: Children::new(vec![
                    html! { <PlaybackNotification message="Playback guidance stays above dialogs." fixed={Some(true)} /> }
                ]),
            }
        ),
    );
}
