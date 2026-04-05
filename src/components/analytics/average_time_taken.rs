use crate::i18n::use_i18n;
use chrono::Duration;
use konnektoren_core::analytics::Metric;
use konnektoren_core::analytics::Trend;
use konnektoren_core::analytics::metrics::AverageTimeTakenMetric;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AverageTimeTakenProps {
    pub metric: AverageTimeTakenMetric,
    #[prop_or(Duration::days(7))]
    pub trend_window: Duration,
}

// Clamp value to a gauge width: fast (<10s) fills near 100%, slow (>120s) near 100% red.
// We express speed as a percentage of a 120s cap so faster = higher fill.
fn gauge_width(value: f64) -> f64 {
    let capped = value.min(120.0);
    (capped / 120.0 * 100.0).clamp(2.0, 100.0)
}

fn get_speed_modifier(value: f64) -> &'static str {
    if value <= 10.0 {
        "average-time-taken__gauge-fill--fast"
    } else if value <= 30.0 {
        "average-time-taken__gauge-fill--medium"
    } else {
        "average-time-taken__gauge-fill--slow"
    }
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
                <div class="average-time-taken__value-container">
                    <div class="average-time-taken__value">
                        <span class="average-time-taken__value-number">
                            { format!("{:.0}", value) }
                        </span>
                        <span class="average-time-taken__value-label">{ i18n.t("seconds") }</span>
                    </div>
                    <div class="average-time-taken__gauge">
                        <div
                            class={classes!("average-time-taken__gauge-fill", get_speed_modifier(value))}
                            style={format!("width: {}%", gauge_width(value))}
                        />
                    </div>
                </div>
                <div class="average-time-taken__details">
                    <p class="average-time-taken__description">
                        { i18n.t(props.metric.description()) }
                    </p>
                </div>
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
    use yew_preview::test_utils::{exists, has_class, has_text};

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

    fn create_challenge_at(duration_secs: i64, days_ago: i64) -> Challenge {
        let end = Utc::now() - Duration::days(days_ago);
        let mut challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge.start_time = Some(end - Duration::seconds(duration_secs));
        challenge.end_time = Some(end);
        challenge
    }

    fn create_improving_metric() -> AverageTimeTakenMetric {
        // Older (>7 days ago): slow ~60s; recent (<7 days): fast ~10s
        let mut history = ChallengeHistory::new();
        history.add_challenge(create_challenge_at(60, 10));
        history.add_challenge(create_challenge_at(55, 9));
        history.add_challenge(create_challenge_at(10, 2));
        history.add_challenge(create_challenge_at(12, 1));
        AverageTimeTakenMetric::new(history)
    }

    fn create_declining_metric() -> AverageTimeTakenMetric {
        // Older (>7 days ago): fast ~10s; recent (<7 days): slow ~60s
        let mut history = ChallengeHistory::new();
        history.add_challenge(create_challenge_at(10, 10));
        history.add_challenge(create_challenge_at(12, 9));
        history.add_challenge(create_challenge_at(60, 2));
        history.add_challenge(create_challenge_at(55, 1));
        AverageTimeTakenMetric::new(history)
    }

    fn create_single_challenge_metric() -> AverageTimeTakenMetric {
        create_metric_with_challenges(vec![15])
    }

    yew_preview::create_preview_with_tests!(
        component: AverageTimeTakenComponent,
        default_props: AverageTimeTakenProps {
            metric: create_medium_metric(),
            trend_window: Duration::days(7),
        },
        variants: [
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
            ),
        ],
        tests: [
            ("Has average-time-taken wrapper", has_class("average-time-taken")),
            ("Shows seconds unit", has_text("seconds")),
            ("Shows trend label", exists("average-time-taken__trend")),
            ("Shows value element", exists("average-time-taken__value")),
            ("Shows description", exists("average-time-taken__description")),
        ]
    );
}
