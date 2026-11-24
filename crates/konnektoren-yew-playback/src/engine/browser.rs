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
}

/// Activate a target, or just resolve it for a visual tap. Returns `false` if
/// an optional target is absent. Invalid selectors are always errors.
pub fn click_browser_target(
    selector: &str,
    optional: bool,
    tap: bool,
) -> Result<bool, BrowserTargetError> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(BrowserTargetError::NoDocument)?;
    let element = document
        .query_selector(selector)
        .map_err(|_| BrowserTargetError::InvalidSelector(selector.into()))?;
    let Some(element) = element else {
        return if optional {
            Ok(false)
        } else {
            Err(BrowserTargetError::MissingTarget(selector.into()))
        };
    };
    let element = element
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| BrowserTargetError::NotHtml(selector.into()))?;
    if !tap {
        element.click();
    }
    Ok(true)
}
