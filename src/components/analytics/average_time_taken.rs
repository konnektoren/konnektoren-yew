use crate::i18n::use_i18n;
use chrono::Duration;
use konnektoren_core::analytics::Metric;
use konnektoren_core::analytics::Trend;
use konnektoren_core::analytics::metrics::AverageTimeTakenMetric;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AverageTimeTakenProps {
    pub metric: AverageTimeTakenMetric,
    #[prop_or(Duration::days(7))]
    pub trend_window: Duration,
}

#[function_component(AverageTimeTakenComponent)]
pub fn average_time_taken(props: &AverageTimeTakenProps) -> Html {
    let i18n = use_i18n();
    let value = props.metric.value();
    let trend = props.metric.get_trend(props.trend_window);

    html! {
        <div class="average-time-taken">
            <div class="average-time-taken__header">
                <h3 class="average-time-taken__title">{ i18n.t(props.metric.name()) }</h3>
                <div class={classes!("average-time-taken__trend", get_trend_modifier(&trend))}>
                    <i class={classes!("average-time-taken__trend-icon", "fas", get_trend_icon(&trend))}></i>
                    <span class="average-time-taken__trend-label">{ i18n.t(&trend.to_string()) }</span>
                </div>
            </div>
            <div class="average-time-taken__content">
                <p class="average-time-taken__value">
                    { format!("{} {}", value, i18n.t("seconds")) }
                </p>
                <p class="average-time-taken__description">
                    { i18n.t(props.metric.description()) }
                </p>
            </div>
        </div>
    }
}

fn get_trend_modifier(trend: &Trend) -> &'static str {
    match trend {
        Trend::Improving => "average-time-taken__trend--improving",
        Trend::Declining => "average-time-taken__trend--declining",
        Trend::Stable => "average-time-taken__trend--stable",
    }
}

fn get_trend_icon(trend: &Trend) -> &'static str {
    match trend {
        Trend::Improving => "fa-arrow-up",
        Trend::Declining => "fa-arrow-down",
        Trend::Stable => "fa-arrows-left-right",
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use chrono::{Duration, Utc};
    use konnektoren_core::challenges::ChallengeHistory;
    use yew_preview::prelude::*;

    fn create_metric() -> AverageTimeTakenMetric {
        // Create a mock history that will result in the desired value and trend
        let mut history = ChallengeHistory::new();
        // Mock some challenges.  Make sure elapsed time is set correctly!
        // Challenge 1:  5 seconds
        let mut challenge1 = konnektoren_core::challenges::Challenge::new(
            &konnektoren_core::challenges::ChallengeType::default(),
            &konnektoren_core::challenges::ChallengeConfig::default(),
        );
        challenge1.start_time = Some(Utc::now() - Duration::seconds(5));
        challenge1.end_time = Some(Utc::now());
        history.add_challenge(challenge1);

        // Challenge 2: 10 seconds
        let mut challenge2 = konnektoren_core::challenges::Challenge::new(
            &konnektoren_core::challenges::ChallengeType::default(),
            &konnektoren_core::challenges::ChallengeConfig::default(),
        );
        challenge2.start_time = Some(Utc::now() - Duration::seconds(10));
        challenge2.end_time = Some(Utc::now());
        history.add_challenge(challenge2);

        AverageTimeTakenMetric::new(history)
    }

    yew_preview::create_preview!(
        AverageTimeTakenComponent,
        AverageTimeTakenProps {
            metric: create_metric(),
            trend_window: Duration::days(7),
        },
        (
            "empty",
            AverageTimeTakenProps {
                metric: AverageTimeTakenMetric::new(ChallengeHistory::new()),
                trend_window: Duration::days(7),
            }
        ),
    );
}
