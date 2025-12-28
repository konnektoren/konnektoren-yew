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
    use konnektoren_core::challenges::{
        Challenge, ChallengeConfig, ChallengeHistory, ChallengeType,
    };
    use yew_preview::prelude::*;

    fn create_metric_with_challenges(challenge_times: Vec<i64>) -> AverageTimeTakenMetric {
        let mut history = ChallengeHistory::new();

        for time in challenge_times {
            let mut challenge =
                Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
            challenge.start_time = Some(Utc::now() - Duration::seconds(time));
            challenge.end_time = Some(Utc::now());
            history.add_challenge(challenge);
        }

        AverageTimeTakenMetric::new(history)
    }

    fn create_fast_metric() -> AverageTimeTakenMetric {
        // Fast completion times: 5, 8, 6, 7 seconds (avg ~6.5s)
        create_metric_with_challenges(vec![5, 8, 6, 7])
    }

    fn create_medium_metric() -> AverageTimeTakenMetric {
        // Medium completion times: 20, 25, 18, 22 seconds (avg ~21s)
        create_metric_with_challenges(vec![20, 25, 18, 22])
    }

    fn create_slow_metric() -> AverageTimeTakenMetric {
        // Slow completion times: 60, 75, 55, 80 seconds (avg ~67.5s)
        create_metric_with_challenges(vec![60, 75, 55, 80])
    }

    fn create_improving_metric() -> AverageTimeTakenMetric {
        // Times getting faster: 60, 50, 40, 30, 20 seconds
        create_metric_with_challenges(vec![60, 50, 40, 30, 20])
    }

    fn create_declining_metric() -> AverageTimeTakenMetric {
        // Times getting slower: 10, 20, 30, 40, 50 seconds
        create_metric_with_challenges(vec![10, 20, 30, 40, 50])
    }

    fn create_single_challenge_metric() -> AverageTimeTakenMetric {
        create_metric_with_challenges(vec![15])
    }

    yew_preview::create_preview!(
        AverageTimeTakenComponent,
        AverageTimeTakenProps {
            metric: create_medium_metric(),
            trend_window: Duration::days(7),
        },
        (
            "Fast Performance (< 10s)",
            AverageTimeTakenProps {
                metric: create_fast_metric(),
                trend_window: Duration::days(7),
            }
        ),
        (
            "Slow Performance (> 60s)",
            AverageTimeTakenProps {
                metric: create_slow_metric(),
                trend_window: Duration::days(7),
            }
        ),
        (
            "Improving Trend",
            AverageTimeTakenProps {
                metric: create_improving_metric(),
                trend_window: Duration::days(7),
            }
        ),
        (
            "Declining Trend",
            AverageTimeTakenProps {
                metric: create_declining_metric(),
                trend_window: Duration::days(7),
            }
        ),
        (
            "Single Challenge",
            AverageTimeTakenProps {
                metric: create_single_challenge_metric(),
                trend_window: Duration::days(7),
            }
        ),
        (
            "Empty (No Challenges)",
            AverageTimeTakenProps {
                metric: AverageTimeTakenMetric::new(ChallengeHistory::new()),
                trend_window: Duration::days(7),
            }
        )
    );
}
