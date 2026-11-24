use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackExitProps {
    /// Translated button label.
    pub label: AttrValue,
    pub on_exit: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    /// Keep the button usable while the page shows a modal dialog, by moving it
    /// into the dialog or else into a viewport overlay.
    #[prop_or_default]
    pub above_dialogs: bool,
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

    let dialog = use_modal_dialog(props.above_dialogs);
    let button = html! {
        <button type="button" class={classes!("playback-exit", props.class.clone())}
            onclick={props.on_exit.reform(|_: MouseEvent| ())} aria-keyshortcuts="Escape">
            {props.label.clone()} <kbd aria-hidden="true">{"Esc"}</kbd>
        </button>
    };
    match dialog {
        #[cfg(feature = "csr")]
        Some(dialog) => yew::create_portal(button, dialog),
        #[cfg(not(feature = "csr"))]
        Some(()) => button,
        None if props.above_dialogs => html! {
            <crate::PlaybackOverlay step={0} viewport={Some(true)}>{button}</crate::PlaybackOverlay>
        },
        None => button,
    }
}

#[cfg(feature = "csr")]
type ModalDialog = web_sys::Element;
#[cfg(not(feature = "csr"))]
type ModalDialog = ();

/// The open modal dialog, polled while `enabled`.
#[allow(clippy::clone_on_copy)]
#[hook]
fn use_modal_dialog(enabled: bool) -> Option<ModalDialog> {
    let dialog = use_state(|| None::<ModalDialog>);
    #[cfg(feature = "csr")]
    {
        let dialog = dialog.clone();
        use_effect_with(enabled, move |enabled| {
            let timer = enabled.then(|| {
                let mut current = None::<web_sys::Element>;
                gloo_timers::callback::Interval::new(100, move || {
                    let active = web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| {
                            document.query_selector("dialog:modal").ok().flatten()
                        });
                    if current != active {
                        current = active.clone();
                        dialog.set(active);
                    }
                })
            });
            move || drop(timer)
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = enabled;
    (*dialog).clone()
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackExit,
        PlaybackExitProps {
            label: "Exit playback".into(),
            on_exit: Callback::noop(),
            class: Classes::new(),
            above_dialogs: false,
        },
    );
}
