use yew::prelude::*;

/// Dismiss navigation overlays on outside pointer interaction or Escape.
#[hook]
pub(super) fn use_navigation_dismiss(root: NodeRef, open: UseStateHandle<bool>) {
    #[cfg(feature = "csr")]
    {
        use wasm_bindgen::JsCast;
        use_effect_with((root, *open), move |(root, is_open)| {
            let listeners = if *is_open {
                web_sys::window()
                    .and_then(|window| window.document())
                    .map(|document| {
                        let root = root.clone();
                        let close = open.clone();
                        let pointer = gloo::events::EventListener::new(
                            &document,
                            "pointerdown",
                            move |event| {
                                if let Some(element) = root.cast::<web_sys::Element>()
                                    && let Some(target) = event
                                        .target()
                                        .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                                    && !element.contains(Some(&target))
                                {
                                    close.set(false);
                                }
                            },
                        );
                        let close = open.clone();
                        let keyboard =
                            gloo::events::EventListener::new(&document, "keydown", move |event| {
                                if let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>()
                                    && event.key() == "Escape"
                                {
                                    close.set(false);
                                }
                            });
                        (pointer, keyboard)
                    })
            } else {
                None
            };
            move || drop(listeners)
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = (root, open);
}
