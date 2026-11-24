use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackExitProps {
    /// Translated button label.
    pub label: AttrValue,
    pub on_exit: Callback<()>,
}

/// Lets the host stop playback using a button or Escape.
#[function_component(PlaybackExit)]
pub fn playback_exit(props: &PlaybackExitProps) -> Html {
    #[cfg(feature = "csr")]
    {
        use wasm_bindgen::JsCast;

        let on_exit = props.on_exit.clone();
        use_effect_with(on_exit, |on_exit| {
            let on_exit = on_exit.clone();
            let listener = web_sys::window().map(|window| {
                gloo_events::EventListener::new_with_options(
                    &window,
                    "keydown",
                    gloo_events::EventListenerOptions {
                        phase: gloo_events::EventListenerPhase::Capture,
                        passive: false,
                    },
                    move |event| {
                        if let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>()
                            && event.key() == "Escape"
                            && !event.repeat()
                        {
                            event.prevent_default();
                            on_exit.emit(());
                        }
                    },
                )
            });
            move || drop(listener)
        });
    }

    html! {
        <button type="button" class="playback-exit"
            onclick={props.on_exit.reform(|_: MouseEvent| ())} aria-keyshortcuts="Escape">
            {props.label.clone()} <kbd aria-hidden="true">{"Esc"}</kbd>
        </button>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackExit,
        PlaybackExitProps {
            label: "Exit playback".into(),
            on_exit: Callback::noop()
        },
    );
}
