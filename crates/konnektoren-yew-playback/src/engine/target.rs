use crate::TargetRect;
use yew::prelude::*;

/// Track a selector in viewport coordinates. When `follow` is enabled, the
/// target is periodically rechecked and scrolled into view when it moves away.
/// Missing targets yield `None`; SSR never accesses the browser.
#[hook]
pub fn use_playback_target(selector: Option<AttrValue>, follow: bool) -> Option<TargetRect> {
    let target = use_state_eq(|| None);
    #[cfg(feature = "csr")]
    {
        let target = target.clone();
        use_effect_with((selector, follow), move |(selector, follow)| {
            let selector = selector.clone();
            let follow = *follow;
            target.set(None);
            let found = std::rc::Rc::new(std::cell::Cell::new(false));
            let update = std::rc::Rc::new(move || {
                let element = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| {
                        selector
                            .as_ref()
                            .and_then(|selector| document.query_selector(selector).ok().flatten())
                    });
                let Some(element) = element else {
                    found.set(false);
                    target.set(None);
                    return;
                };
                let bounds = element.get_bounding_client_rect();
                let outside = web_sys::window().is_some_and(|window| {
                    let width = window
                        .inner_width()
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or_default();
                    let height = window
                        .inner_height()
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or_default();
                    width > 0.0
                        && height > 0.0
                        && (bounds.bottom() < 0.0
                            || bounds.top() > height
                            || bounds.right() < 0.0
                            || bounds.left() > width)
                });
                if !found.replace(true) || (follow && outside) {
                    element.scroll_into_view();
                }
                target.set(Some(TargetRect {
                    x: bounds.x(),
                    y: bounds.y(),
                    width: bounds.width(),
                    height: bounds.height(),
                }));
            });
            update();
            let interval = follow.then(|| {
                let update = update.clone();
                gloo_timers::callback::Interval::new(100, move || update())
            });
            let listeners = web_sys::window().map(|window| {
                ["scroll", "resize"].map(|event| {
                    let update = update.clone();
                    gloo_events::EventListener::new(&window, event, move |_| update())
                })
            });
            move || {
                drop(interval);
                drop(listeners);
            }
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = (selector, follow);
    *target
}
