use yew::prelude::*;

use super::AnalyticsContext;

#[derive(Properties, PartialEq)]
pub struct WebAnalyticsProviderProps {
    pub backend: AnalyticsContext,
    pub children: Children,
}
