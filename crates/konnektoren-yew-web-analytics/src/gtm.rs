use crate::{Parameters, TrackError, TrackResult, WebAnalytics};
#[cfg(all(feature = "csr", target_arch = "wasm32"))]
use wasm_bindgen::{JsCast, JsValue};

/// Google Tag Manager data layer backend.
///
/// The host must load GTM and set `window.konnektorenAnalyticsConsent` when
/// analytics consent changes. Events before consent are dropped.
#[derive(Clone, Copy, Debug, Default)]
pub struct GtmAnalytics;

impl WebAnalytics for GtmAnalytics {
    fn track_event(&self, name: &str, mut parameters: Parameters) -> TrackResult {
        parameters.insert("event".into(), name.into());
        push_event(parameters)
    }
}

/// Track an event using the GTM backend. Success means it was queued in the
/// data layer; GTM must still forward it to the configured destination.
pub fn track_event(name: &str, parameters: Parameters) -> TrackResult {
    GtmAnalytics.track_event(name, parameters)
}

#[cfg(not(all(feature = "csr", target_arch = "wasm32")))]
fn push_event(_: Parameters) -> TrackResult {
    Err(TrackError::Unavailable)
}

#[cfg(all(feature = "csr", target_arch = "wasm32"))]
fn push_event(parameters: Parameters) -> TrackResult {
    let Some(window) = web_sys::window() else {
        return Err(TrackError::Unavailable);
    };
    let get = |key: &str| js_sys::Reflect::get(&window, &JsValue::from_str(key));
    match get("konnektorenAnalyticsConsent").ok().and_then(|v| v.as_bool()) {
        Some(true) => {},
        Some(false) => return Err(TrackError::ConsentDenied),
        None => {
            tracing::warn!("GTM analytics consent configuration is missing: expected window.konnektorenAnalyticsConsent");
            return Err(TrackError::Unavailable);
        }
    }
    let Ok(layer) = get("dataLayer") else {
        tracing::warn!("GTM data layer is unavailable; check the host page analytics configuration");
        return Err(TrackError::Unavailable);
    };
    let Ok(push) = js_sys::Reflect::get(&layer, &JsValue::from_str("push")) else {
        tracing::warn!("GTM data layer is unavailable; check the host page analytics configuration");
        return Err(TrackError::Unavailable);
    };
    let Some(push) = push.dyn_ref::<js_sys::Function>() else {
        tracing::warn!("GTM data layer is missing a push function; check the host page analytics configuration");
        return Err(TrackError::Unavailable);
    };
    let Ok(event) = js_sys::JSON::parse(&serde_json::Value::Object(parameters).to_string()) else {
        return Err(TrackError::Unavailable);
    };
    if push.call1(&layer, &event).is_ok() {
        Ok(())
    } else {
        Err(TrackError::Unavailable)
    }
}
