use crate::i18n::use_i18n;
use chrono::Duration;
use konnektoren_core::analytics::Metric;
use konnektoren_core::analytics::Trend;
use konnektoren_core::analytics::metrics::SuccessRateMetric;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SuccessRateProps {
    pub metric: SuccessRateMetric,
    #[prop_or(Duration::days(7))]
    pub trend_window: Duration,
}

#[function_component(SuccessRateComponent)]
pub fn success_rate(props: &SuccessRateProps) -> Html {
    let i18n = use_i18n();
    let value = props.metric.value();
    let trend = props.metric.get_trend(props.trend_window);

    html! {
        <div class="success-rate">
            <div class="success-rate__header">
                <h3 class="success-rate__title">{ i18n.t(props.metric.name()) }</h3>
                <div class={classes!("success-rate__trend", get_trend_modifier(&trend))}>
                    <i class={classes!("success-rate__trend-icon", "fas", get_trend_icon(&trend))}></i>
                    <span class="success-rate__trend-label">{ i18n.t(&trend.to_string()) }</span>
                </div>
            </div>

            <div class="success-rate__content">
                <div class="success-rate__value-container">
                    <div class="success-rate__value">
                        <span class="success-rate__percentage">
                            { format!("{:.1}%", value) }
                        </span>
                        <span class="success-rate__label">{ i18n.t("Success Rate") }</span>
                    </div>
                    <div class="success-rate__gauge">
                        <div
                            class="success-rate__gauge-fill"
                            style={format!("width: {}%", value)}
                        />
                    </div>
                </div>

                <div class="success-rate__details">
                    <p class="success-rate__description">
                        { i18n.t(&props.metric.description()) }
                    </p>
                    <SuccessRateThresholds />
                </div>
            </div>
        </div>
    }
}

fn get_trend_modifier(trend: &Trend) -> &'static str {
    match trend {
        Trend::Improving => "success-rate__trend--improving",
        Trend::Declining => "success-rate__trend--declining",
        Trend::Stable => "success-rate__trend--stable",
    }
}

fn get_trend_icon(trend: &Trend) -> &'static str {
    match trend {
        Trend::Improving => "fa-arrow-trend-up",
        Trend::Declining => "fa-arrow-trend-down",
        Trend::Stable => "fa-arrows-left-right",
    }
}

#[function_component(SuccessRateThresholds)]
fn success_rate_thresholds() -> Html {
    let i18n = use_i18n();
    html! {
        <div class="success-rate__thresholds">
            <div class="success-rate__threshold">
                <span class="success-rate__threshold-marker success-rate__threshold-marker--high">{"80%"}</span>
                <span class="success-rate__threshold-label">{ i18n.t("High Performance") }</span>
            </div>
            <div class="success-rate__threshold">
                <span class="success-rate__threshold-marker success-rate__threshold-marker--medium">{"60%"}</span>
                <span class="success-rate__threshold-label">{ i18n.t("Medium Performance") }</span>
            </div>
            <div class="success-rate__threshold">
                <span class="success-rate__threshold-marker success-rate__threshold-marker--low">{"40%"}</span>
                <span class="success-rate__threshold-label">{ i18n.t("Needs Improvement") }</span>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::ChallengeHistory;
    use yew_preview::prelude::*;

    fn create_metric_with_value(_value: f64, _trend: Trend) -> SuccessRateMetric {
        let history = ChallengeHistory::new();
        SuccessRateMetric::new(history)
    }

    yew_preview::create_preview!(
        SuccessRateComponent,
        SuccessRateProps {
            metric: create_metric_with_value(75.0, Trend::Stable),
            trend_window: Duration::days(7),
        },
        (
            "empty",
            SuccessRateProps {
                metric: SuccessRateMetric::new(ChallengeHistory::new()),
                trend_window: Duration::days(7),
            }
        ),
    );
}
