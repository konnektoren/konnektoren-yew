/// Event parameters, including strings, numbers, booleans, arrays and objects.
pub type Parameters = serde_json::Map<String, serde_json::Value>;

/// Why an event could not be queued in the browser.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TrackError {
    /// Analytics consent has not been granted.
    #[error("analytics consent has not been granted")]
    ConsentDenied,
    /// The analytics backend is unavailable or rejected the event.
    #[error("the analytics backend is unavailable or rejected the event")]
    Unavailable,
}

/// Success means the backend accepted the event for delivery.
pub type TrackResult = Result<(), TrackError>;

/// A backend that accepts named web analytics events.
pub trait WebAnalytics {
    /// Track an event with its parameters.
    fn track_event(&self, name: &str, parameters: Parameters) -> TrackResult;
}
