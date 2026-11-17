use yew::prelude::*;

use super::AnalyticsContext;

/// Returns the configured backend, or `None` when no provider is present.
#[hook]
pub fn use_web_analytics() -> Option<AnalyticsContext> {
    use_context::<AnalyticsContext>()
}
