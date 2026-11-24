use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackOverlayProps {
    /// Reopen the overlay when the step changes, after a page may open a dialog.
    pub step: usize,
    pub children: Children,
}

/// Places playback guidance above native modal dialogs.
#[function_component(PlaybackOverlay)]
pub fn playback_overlay(props: &PlaybackOverlayProps) -> Html {
    let node = use_node_ref();
    #[cfg(feature = "csr")]
    {
        let node = node.clone();
        use_effect_with(props.step, move |_| {
            let timer = gloo_timers::callback::Timeout::new(0, move || {
                if let Some(element) = node.cast::<web_sys::HtmlElement>() {
                    let _ = element.hide_popover();
                    let _ = element.show_popover();
                }
            });
            move || drop(timer)
        });
    }
    html! {
        <div ref={node} class="playback-overlay" popover={if cfg!(feature = "csr") { Some("manual") } else { None }}>
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
            children: Children::new(vec![
                html! { <PlaybackNotification message="Playback guidance stays above dialogs." /> }
            ]),
        },
    );
}
