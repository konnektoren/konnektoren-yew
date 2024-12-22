use crate::components::TimerComponent;
use crate::i18n::use_i18n;
use gloo::net::http::Request;
use konnektoren_core::challenges::{PerformanceRecord, Timed};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardProps {
    #[prop_or_default]
    pub leaderboard_id: Option<String>,

    #[prop_or_default]
    pub default_record: Option<PerformanceRecord>,

    pub api_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct LeaderboardV1Response {
    pub performance_records: Vec<PerformanceRecord>,
}

pub async fn fetch_all_performance_records(
    api_url: &str,
    leaderboard_id: Option<String>,
) -> Result<Vec<PerformanceRecord>, gloo::net::Error> {
    let url = match leaderboard_id {
        Some(id) => format!("{}/{}", api_url, id),
        None => api_url.to_string(),
    };

    let response = Request::get(&url).send().await?;

    let leaderboard: LeaderboardV1Response = response.json().await?;
    let mut performance_records = leaderboard.performance_records;
    performance_records.sort();
    Ok(performance_records)
}

#[function_component(LeaderboardComp)]
pub fn leaderboard_comp(props: &LeaderboardProps) -> Html {
    let i18n = use_i18n();
    let leaderboard = use_state(|| match props.default_record.clone() {
        Some(record) => vec![record],
        None => vec![],
    });

    {
        let default_record = props.default_record.clone();
        let leaderboard = leaderboard.clone();
        let leaderboard_id = props.leaderboard_id.clone();
        let api_url = props.api_url.clone();

        use_effect_with(leaderboard_id.clone(), |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_all_performance_records(&api_url, leaderboard_id).await {
                    Ok(mut performance_records) => {
                        if let Some(default_record) = default_record {
                            performance_records.push(default_record);
                        }
                        performance_records.sort();
                        leaderboard.set(performance_records);
                    }
                    Err(err) => {
                        // Handle error appropriately
                        log::error!("Failed to fetch leaderboard: {:?}", err);
                    }
                }
            });
        });
    }

    if leaderboard.is_empty() {
        return html! {
            <div class="leaderboard">
            </div>
        };
    }

    html! {
        <div class="leaderboard">
            <div class="leaderboard__container">
                <table class="leaderboard__table">
                    <thead class="leaderboard__header">
                        <tr>
                            <th>{i18n.t("Rank")}</th>
                            <th>{i18n.t("Name")}</th>
                            <th>{i18n.t("Performance")}</th>
                            <th>{i18n.t("Time")}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for leaderboard.iter().enumerate().map(|(i, record)| {
                            let elapsed_time = record.elapsed_time().unwrap_or_default().num_milliseconds();
                            html! {
                                <tr class={classes!(
                                    "leaderboard__row",
                                    record.eq(props.default_record.as_ref().unwrap_or(&PerformanceRecord::default()))
                                        .then_some("leaderboard__row--highlighted")
                                )}>
                                    <td class="leaderboard__cell leaderboard__cell--rank">{i + 1}</td>
                                    <td class="leaderboard__cell leaderboard__cell--name">{&record.profile_name}</td>
                                    <td class="leaderboard__cell leaderboard__cell--performance">
                                        {format!("{:.2}%", record.performance_percentage)}
                                    </td>
                                    <td class="leaderboard__cell leaderboard__cell--time">
                                        <TimerComponent milliseconds={elapsed_time} show_milliseconds={false} />
                                    </td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        LeaderboardComp,
        LeaderboardProps {
            leaderboard_id: Some("articles-1".to_string()),
            default_record: Some(PerformanceRecord {
                profile_name: "Test User".to_string(),
                performance_percentage: 0,
                ..Default::default()
            }),
            api_url: "https://api.konnektoren.help/api/v1/leaderboard".to_string(),
        },
        (
            "empty",
            LeaderboardProps {
                leaderboard_id: Some("1".to_string()),
                default_record: None,
                api_url: "https://api.konnektoren.help/api/v1/leaderboard".to_string(),
            }
        ),
    );
}
