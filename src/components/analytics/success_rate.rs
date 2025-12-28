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
    use chrono::{Duration, Utc};
    use konnektoren_core::challenges::{
        ChallengeHistory, ChallengeType, MultipleChoice, MultipleChoiceOption, Question,
    };
    use konnektoren_core::prelude::{Challenge, ChallengeConfig, ChallengeResult};
    use yew_preview::prelude::*;

    fn create_mc_challenge(id: &str, correct_answers: usize, total: usize) -> Challenge {
        let options = vec![
            MultipleChoiceOption {
                id: 1,
                name: "Answer A".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Answer B".to_string(),
            },
            MultipleChoiceOption {
                id: 3,
                name: "Answer C".to_string(),
            },
        ];

        let mut questions = vec![];
        for i in 0..total {
            questions.push(Question {
                question: format!("Question {}?", i + 1),
                help: format!("Help for question {}", i + 1),
                option: 1, // Correct answer is option 1
                image: None,
            });
        }

        let mc = MultipleChoice {
            id: id.to_string(),
            name: format!("Challenge {}", id),
            lang: "en".to_string(),
            options,
            questions,
        };

        // Create result - just the selected options
        let mut result_options = vec![];
        for i in 0..total {
            let user_answer = if i < correct_answers { 1 } else { 2 }; // 1 = correct, 2 = wrong
            result_options.push(MultipleChoiceOption {
                id: user_answer,
                name: format!("Selected answer {}", user_answer),
            });
        }

        let mut challenge = Challenge::new(
            &ChallengeType::MultipleChoice(mc),
            &ChallengeConfig {
                id: id.to_string(),
                ..ChallengeConfig::default()
            },
        );

        challenge.challenge_result = ChallengeResult::MultipleChoice(result_options);
        challenge.start_time = Some(Utc::now());
        challenge.end_time = Some(Utc::now());

        challenge
    }

    fn high_performance_history() -> ChallengeHistory {
        let mut history = ChallengeHistory::new();
        history.add_challenge(create_mc_challenge("high-1", 9, 10)); // 90%
        history.add_challenge(create_mc_challenge("high-2", 10, 10)); // 100%
        history.add_challenge(create_mc_challenge("high-3", 9, 10)); // 90%
        history.add_challenge(create_mc_challenge("high-4", 8, 10)); // 80%
        history
    }

    fn low_performance_history() -> ChallengeHistory {
        let mut history = ChallengeHistory::new();
        history.add_challenge(create_mc_challenge("low-1", 3, 10)); // 30%
        history.add_challenge(create_mc_challenge("low-2", 4, 10)); // 40%
        history.add_challenge(create_mc_challenge("low-3", 2, 10)); // 20%
        history.add_challenge(create_mc_challenge("low-4", 3, 10)); // 30%
        history
    }

    fn perfect_performance_history() -> ChallengeHistory {
        let mut history = ChallengeHistory::new();
        history.add_challenge(create_mc_challenge("perfect-1", 10, 10)); // 100%
        history.add_challenge(create_mc_challenge("perfect-2", 10, 10)); // 100%
        history.add_challenge(create_mc_challenge("perfect-3", 10, 10)); // 100%
        history
    }

    yew_preview::create_preview!(
        SuccessRateComponent,
        SuccessRateProps {
            metric: SuccessRateMetric::new(low_performance_history()),
            trend_window: Duration::days(7),
        },
        (
            "High Performance (100%)",
            SuccessRateProps {
                metric: SuccessRateMetric::new(high_performance_history()),
                trend_window: Duration::days(7),
            }
        ),
        (
            "Empty History",
            SuccessRateProps {
                metric: SuccessRateMetric::new(ChallengeHistory::new()),
                trend_window: Duration::days(7),
            }
        )
    );
}
