use std::rc::Rc;

use crate::{Parameters, TrackResult, WebAnalytics};

/// Shared application analytics backend.
#[derive(Clone)]
pub struct AnalyticsContext(pub Rc<dyn WebAnalytics>);

impl AnalyticsContext {
    /// Wrap an analytics backend for use in a provider.
    pub fn new(backend: impl WebAnalytics + 'static) -> Self {
        Self(Rc::new(backend))
    }
}

impl PartialEq for AnalyticsContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl WebAnalytics for AnalyticsContext {
    fn track_event(&self, name: &str, parameters: Parameters) -> TrackResult {
        self.0.track_event(name, parameters)
    }
}
