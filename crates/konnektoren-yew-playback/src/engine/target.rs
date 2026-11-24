use crate::TargetRect;
use yew::prelude::*;

/// A tracked target in viewport coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaybackTarget {
    pub rect: TargetRect,
    /// Target center as a percentage of the viewport, for [`crate::PlaybackCursor`].
    pub percent: (f64, f64),
}

/// Track a selector in viewport coordinates.
///
/// The target is scrolled to the middle of the screen once found. With `follow`
/// it is rechecked periodically and scrolled back into view when it moves away.
/// A vanished target yields `None`. Changing `step` re-runs the initial scroll,
/// even for the same selector. SSR never accesses the browser.
#[hook]
pub fn use_playback_target(
    selector: Option<AttrValue>,
    follow: bool,
    step: usize,
) -> Option<PlaybackTarget> {
    let target = use_state_eq(|| None);
    #[cfg(feature = "csr")]
    {
        let target = target.clone();
        use_effect_with((selector, follow, step), move |(selector, follow, _)| {
            let selector = selector.clone();
            let follow = *follow;
            let found = std::rc::Rc::new(std::cell::Cell::new(false));
            let update = std::rc::Rc::new(move || {
                let Some(window) = web_sys::window() else {
                    return;
                };
                let element = window.document().and_then(|document| {
                    selector
                        .as_ref()
                        .and_then(|selector| document.query_selector(selector).ok().flatten())
                });
                let Some(element) = element else {
                    // Do not frame a spot where the target used to be.
                    target.set(None);
                    return;
                };
                let (Some(width), Some(height)) = (
                    window.inner_width().ok().and_then(|value| value.as_f64()),
                    window.inner_height().ok().and_then(|value| value.as_f64()),
                ) else {
                    return;
                };
                if width <= 0.0 || height <= 0.0 {
                    return;
                }
                if !found.replace(true) {
                    super::scroll_to_target(&element, true);
                }
                let bounds = element.get_bounding_client_rect();
                if follow
                    && bounds.width() > 0.0
                    && bounds.height() > 0.0
                    && (bounds.bottom() < 0.0
                        || bounds.top() > height
                        || bounds.right() < 0.0
                        || bounds.left() > width)
                {
                    super::scroll_to_target(&element, false);
                }
                let rect = TargetRect {
                    x: bounds.x(),
                    y: bounds.y(),
                    width: bounds.width(),
                    height: bounds.height(),
                };
                let (x, y) = rect.center();
                target.set(Some(PlaybackTarget {
                    rect,
                    percent: (x / width * 100.0, y / height * 100.0),
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
    let _ = (selector, follow, step);
    *target
}
