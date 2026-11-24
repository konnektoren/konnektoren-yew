use wasm_bindgen::JsCast;

/// Errors when resolving or activating a browser target.
#[derive(Debug, thiserror::Error)]
pub enum BrowserTargetError {
    #[error("Browser document is unavailable")]
    NoDocument,
    #[error("Invalid target selector: {0}")]
    InvalidSelector(String),
    #[error("Target was not found: {0}")]
    MissingTarget(String),
    #[error("Target is not an HTML element: {0}")]
    NotHtml(String),
    #[error("Target is not an input element: {0}")]
    NotInput(String),
    #[error("Could not create or dispatch browser event")]
    Event,
}

/// Resolve an HTML target. Optional targets may be absent; malformed selectors
/// and non-HTML targets remain errors.
pub fn resolve_browser_target(
    selector: &str,
    optional: bool,
) -> Result<Option<web_sys::HtmlElement>, BrowserTargetError> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(BrowserTargetError::NoDocument)?;
    let element = document
        .query_selector(selector)
        .map_err(|_| BrowserTargetError::InvalidSelector(selector.into()))?;
    let Some(element) = element else {
        return if optional {
            Ok(None)
        } else {
            Err(BrowserTargetError::MissingTarget(selector.into()))
        };
    };
    let element = element
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| BrowserTargetError::NotHtml(selector.into()))?;
    Ok(Some(element))
}

/// Activate a target, or just resolve it for a visual tap. Returns `false` if
/// an optional target is absent.
pub fn click_browser_target(
    selector: &str,
    optional: bool,
    tap: bool,
) -> Result<bool, BrowserTargetError> {
    let Some(element) = resolve_browser_target(selector, optional)? else {
        return Ok(false);
    };
    if !tap {
        element.click();
    }
    Ok(true)
}

/// Set an input value and notify the application's input handler.
pub fn fill_browser_target(
    selector: &str,
    value: &str,
    optional: bool,
) -> Result<bool, BrowserTargetError> {
    let Some(element) = resolve_browser_target(selector, optional)? else {
        return Ok(false);
    };
    let input = element
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| BrowserTargetError::NotInput(selector.into()))?;
    input.set_value(value);
    let options = web_sys::InputEventInit::new();
    options.set_bubbles(true);
    let event = web_sys::InputEvent::new_with_event_init_dict("input", &options)
        .map_err(|_| BrowserTargetError::Event)?;
    input
        .dispatch_event(&event)
        .map_err(|_| BrowserTargetError::Event)?;
    Ok(true)
}

/// Dispatch one key event to a resolved target. Keep the target for a matching
/// keyup even if the DOM changes while the key is held.
pub fn send_browser_key(
    target: &web_sys::HtmlElement,
    key: &str,
    pressed: bool,
) -> Result<(), BrowserTargetError> {
    let options = web_sys::KeyboardEventInit::new();
    options.set_key(key);
    options.set_bubbles(true);
    options.set_cancelable(true);
    let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
        if pressed { "keydown" } else { "keyup" },
        &options,
    )
    .map_err(|_| BrowserTargetError::Event)?;
    target
        .dispatch_event(&event)
        .map_err(|_| BrowserTargetError::Event)?;
    Ok(())
}
