use super::PlaybackControls;
use yew::prelude::*;

/// Keyboard controls: ← slower, → faster, ↑ run the current step now.
///
/// Keys typed into form fields and shortcuts with modifiers are ignored.
#[hook]
pub fn use_playback_keys(controls: PlaybackControls) {
    #[cfg(feature = "csr")]
    {
        use_effect_with(controls, |controls| {
            use wasm_bindgen::JsCast;

            let controls = controls.clone();
            let listener = web_sys::window().map(|window| {
            gloo_events::EventListener::new(&window, "keydown", move |event| {
                let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
                    return;
                };
                if event.repeat() || event.ctrl_key() || event.alt_key() || event.meta_key() {
                    return;
                }
                if let Some(target) = event
                    .target()
                    .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                    && target
                        .closest("input, textarea, select, [contenteditable]:not([contenteditable='false'])")
                        .ok()
                        .flatten()
                        .is_some()
                {
                    return;
                }
                match event.key().as_str() {
                    "ArrowRight" => controls.faster(),
                    "ArrowLeft" => controls.slower(),
                    "ArrowUp" => controls.next(),
                    _ => return,
                }
                event.prevent_default();
            })
        });
            move || drop(listener)
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = controls;
}
