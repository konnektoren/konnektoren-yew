use yew::prelude::*;

use super::{AnalyticsContext, WebAnalyticsProviderProps};

#[function_component(WebAnalyticsProvider)]
pub fn web_analytics_provider(props: &WebAnalyticsProviderProps) -> Html {
    html! { <ContextProvider<AnalyticsContext> context={props.backend.clone()}>{props.children.clone()}</ContextProvider<AnalyticsContext>> }
}
