use std::{cell::RefCell, rc::Rc};

use konnektoren_yew_web_analytics::{
    AnalyticsContext, Parameters, TrackError, TrackResult, WebAnalytics, WebAnalyticsProvider,
    use_web_analytics,
};
use serde_json::json;
use yew::prelude::*;

#[derive(Clone, Default)]
struct MockAnalytics {
    events: Rc<RefCell<Vec<(String, Parameters)>>>,
    error: Option<TrackError>,
}

impl WebAnalytics for MockAnalytics {
    fn track_event(&self, name: &str, parameters: Parameters) -> TrackResult {
        self.events.borrow_mut().push((name.into(), parameters));
        self.error.map_or(Ok(()), Err)
    }
}

#[test]
fn context_forwards_events_and_backend_errors() {
    let backend = MockAnalytics {
        error: Some(TrackError::ConsentDenied),
        ..Default::default()
    };
    let context = AnalyticsContext::new(backend.clone());
    let mut parameters = Parameters::new();
    parameters.insert("item_id".into(), json!("learn"));
    parameters.insert("count".into(), json!(2));

    assert_eq!(
        context.track_event("favourite_added", parameters.clone()),
        Err(TrackError::ConsentDenied)
    );
    assert_eq!(
        *backend.events.borrow(),
        vec![("favourite_added".into(), parameters)]
    );
    assert!(context == context.clone());
    assert!(context != AnalyticsContext::new(backend));
}

#[function_component(Consumer)]
fn consumer() -> Html {
    let analytics = use_web_analytics();
    let result =
        analytics.map(|backend| backend.track_event("onboarding_complete", Parameters::new()));
    html! { <span>{if result == Some(Ok(())) { "tracked" } else { "unavailable" }}</span> }
}

#[function_component(Host)]
fn host() -> Html {
    let backend = AnalyticsContext::new(MockAnalytics::default());
    html! { <WebAnalyticsProvider {backend}><Consumer /></WebAnalyticsProvider> }
}

#[tokio::test]
async fn provider_supplies_backend_to_descendant_hook() {
    let rendered = yew::ServerRenderer::<Host>::new()
        .hydratable(false)
        .render()
        .await;
    assert!(rendered.contains("tracked"), "{rendered}");
}

#[tokio::test]
async fn hook_returns_none_without_provider() {
    let rendered = yew::ServerRenderer::<Consumer>::new()
        .hydratable(false)
        .render()
        .await;
    assert!(rendered.contains("unavailable"), "{rendered}");
}
