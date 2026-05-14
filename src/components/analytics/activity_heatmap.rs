use crate::i18n::use_i18n;
use chrono::{Datelike, Days, NaiveDate, Utc};
use konnektoren_core::challenges::{Challenge, ChallengeHistory};
use konnektoren_core::prelude::Performance;
use yew::prelude::*;

// ─── Props ────────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub struct ActivityHeatmapProps {
    pub challenge_history: ChallengeHistory,
    /// Number of days to show (default 365)
    #[prop_or(365)]
    pub days: usize,
}

// ─── Day detail modal ─────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct DayDetailModalProps {
    pub date: NaiveDate,
    pub challenges: Vec<Challenge>,
    pub on_close: Callback<()>,
}

#[function_component(DayDetailModal)]
fn day_detail_modal(props: &DayDetailModalProps) -> Html {
    let i18n = use_i18n();

    let title = format!(
        "{} {}",
        i18n.t("Challenges on"),
        props.date.format("%B %d, %Y")
    );
    let close_label = i18n.t("Close");
    let col_challenge = i18n.t("Challenge");
    let col_time = i18n.t("Time");
    let col_result = i18n.t("Result");
    let empty_text = i18n.t("No challenges completed yet.");

    let on_close = props.on_close.clone();
    let on_close_backdrop = props.on_close.clone();

    let rows: Html = props
        .challenges
        .iter()
        .map(|c| {
            let perf = c.performance(&c.challenge_result);
            let badge_mod = if perf >= 80 {
                "badge-success-soft"
            } else if perf >= 60 {
                "badge-warning-soft"
            } else {
                "badge-error-soft"
            };
            let name = c.challenge_type.name();
            let time = c
                .end_time
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_default();
            html! {
                <tr class="table-row">
                    <td class="table-cell">{ name }</td>
                    <td class="table-cell activity-heatmap__modal-time">{ time }</td>
                    <td class="table-cell">
                        <span class={classes!("badge", "badge-sm", badge_mod)}>
                            { format!("{}%", perf) }
                        </span>
                    </td>
                </tr>
            }
        })
        .collect();

    html! {
        <dialog class="modal modal-open">
            <div class="modal-box activity-heatmap__modal-box">
                <h3 class="activity-heatmap__modal-title">{ title }</h3>
                if props.challenges.is_empty() {
                    <p class="empty-state">{ empty_text }</p>
                } else {
                    <div class="overflow-x-auto">
                        <table class="table-base">
                            <thead class="table-header">
                                <tr>
                                    <th class="table-header-cell">{ col_challenge }</th>
                                    <th class="table-header-cell">{ col_time }</th>
                                    <th class="table-header-cell">{ col_result }</th>
                                </tr>
                            </thead>
                            <tbody>{ rows }</tbody>
                        </table>
                    </div>
                }
                <div class="modal-action">
                    <button
                        class="btn btn-ghost btn-sm"
                        onclick={Callback::from(move |_: MouseEvent| on_close.emit(()))}
                    >
                        { close_label }
                    </button>
                </div>
            </div>
            <div
                class="modal-backdrop"
                onclick={Callback::from(move |_: MouseEvent| on_close_backdrop.emit(()))}
            />
        </dialog>
    }
}

// ─── Main component ───────────────────────────────────────────────────────────

#[function_component(ActivityHeatmapComponent)]
pub fn activity_heatmap(props: &ActivityHeatmapProps) -> Html {
    let i18n = use_i18n();
    let selected_date: UseStateHandle<Option<NaiveDate>> = use_state(|| None);

    // All i18n strings extracted upfront — avoids borrow issues inside html! closures
    let title_text = i18n.t("Activity Heatmap");
    let weeks_text = i18n.t("weeks of");
    let learning_text = i18n.t("learning");
    let no_challenges_text = i18n.t("No challenges");
    let challenges_text = i18n.t("challenges");
    let less_text = i18n.t("Less");
    let more_text = i18n.t("More");
    let active_days_text = i18n.t("active days");
    let in_text = i18n.t("in");
    let total_chal_text = i18n.t("total challenges");

    // Translated day-of-week labels — only Mon/Wed/Fri shown, rest empty
    let day_labels: [String; 7] = [
        i18n.t("Mon"),
        String::new(),
        i18n.t("Wed"),
        String::new(),
        i18n.t("Fri"),
        String::new(),
        String::new(),
    ];

    // ── Build date → count map ────────────────────────────────────────────────
    let mut day_counts: std::collections::HashMap<NaiveDate, usize> =
        std::collections::HashMap::new();
    for challenge in &props.challenge_history.challenges {
        if let Some(end_time) = challenge.end_time {
            *day_counts.entry(end_time.date_naive()).or_insert(0) += 1;
        }
    }

    let today = Utc::now().date_naive();
    let start_date = today - Days::new(props.days as u64);

    // ── Build week grid (columns = weeks, rows = Mon–Sun) ─────────────────────
    let mut weeks: Vec<Vec<(NaiveDate, usize)>> = Vec::new();
    let mut cur = start_date;
    let offset = cur.weekday().num_days_from_monday();
    if offset > 0 {
        cur = cur - Days::new(offset as u64);
    }
    while cur <= today {
        let mut week = Vec::with_capacity(7);
        for _ in 0..7 {
            let count = day_counts.get(&cur).copied().unwrap_or(0);
            week.push((cur, count));
            cur = cur + Days::new(1);
        }
        weeks.push(week);
    }

    // ── Per-week month label: Some("Jan") at first week of each month ─────────
    let week_month_labels: Vec<Option<String>> = {
        let mut out = Vec::with_capacity(weeks.len());
        let mut last: Option<(u32, i32)> = None;
        for week in &weeks {
            if let Some((date, _)) = week.first() {
                let key = (date.month(), date.year());
                if last != Some(key) {
                    out.push(Some(date.format("%b").to_string()));
                    last = Some(key);
                } else {
                    out.push(None);
                }
            } else {
                out.push(None);
            }
        }
        out
    };

    let max_count = day_counts.values().copied().max().unwrap_or(0);

    // ── Build modal outside html! to avoid borrow issues ──────────────────────
    let modal_html = if let Some(date) = *selected_date {
        let challenges_for_day: Vec<Challenge> = props
            .challenge_history
            .challenges
            .iter()
            .filter(|c| c.end_time.map(|t| t.date_naive() == date).unwrap_or(false))
            .cloned()
            .collect();
        let close_cb = {
            let sd = selected_date.clone();
            Callback::from(move |_: ()| sd.set(None))
        };
        html! {
            <DayDetailModal
                date={date}
                challenges={challenges_for_day}
                on_close={close_cb}
            />
        }
    } else {
        html! {}
    };

    // ── Render ────────────────────────────────────────────────────────────────
    html! {
        <div class="activity-heatmap">
            { modal_html }

            <div class="activity-heatmap__header">
                <h3 class="activity-heatmap__title">{ title_text }</h3>
                <span class="activity-heatmap__subtitle">
                    { format!("{} {} {}", weeks.len(), weeks_text, learning_text) }
                </span>
            </div>

            <div class="activity-heatmap__grid-wrapper">
                <div class="activity-heatmap__months">
                    <div class="activity-heatmap__day-label-spacer"></div>
                    <div class="activity-heatmap__months-cells">
                        {for week_month_labels.iter().map(|label| {
                            html! {
                                <span class="activity-heatmap__month-cell">
                                    { label.clone().unwrap_or_default() }
                                </span>
                            }
                        })}
                    </div>
                </div>

                <div class="activity-heatmap__grid">
                    {for (0..7usize).map(|day_idx| {
                        let day_label = day_labels[day_idx].clone();
                        let cells = weeks.iter().map(|week| {
                            let (date, count) = week[day_idx];
                            let level     = count_to_level(count, max_count);
                            let is_future = date > today;
                            let clickable = !is_future && count > 0;
                            let title = if count == 0 {
                                format!("{}: {}", no_challenges_text, date.format("%b %d, %Y"))
                            } else {
                                format!("{} {} – {}", count, challenges_text, date.format("%b %d, %Y"))
                            };
                            let sd = selected_date.clone();
                            let onclick = Callback::from(move |_: MouseEvent| {
                                if clickable {
                                    sd.set(Some(date));
                                }
                            });
                            html! {
                                <div
                                    class={classes!(
                                        "activity-heatmap__cell",
                                        format!("activity-heatmap__cell--level-{}", level),
                                        is_future.then_some("activity-heatmap__cell--future"),
                                        clickable.then_some("activity-heatmap__cell--clickable"),
                                    )}
                                    title={title}
                                    onclick={onclick}
                                />
                            }
                        }).collect::<Html>();

                        html! {
                            <div class="activity-heatmap__row">
                                <span class="activity-heatmap__day-label">
                                    { day_label }
                                </span>
                                <div class="activity-heatmap__cells">{ cells }</div>
                            </div>
                        }
                    })}
                </div>
            </div>

            <div class="activity-heatmap__footer">
                <div class="activity-heatmap__legend">
                    <span class="activity-heatmap__legend-label">{ less_text }</span>
                    {for (0..5usize).map(|level| {
                        html! {
                            <div class={classes!(
                                "activity-heatmap__legend-cell",
                                format!("activity-heatmap__cell--level-{}", level),
                            )} />
                        }
                    })}
                    <span class="activity-heatmap__legend-label">{ more_text }</span>
                </div>
                <div class="activity-heatmap__stats">
                    <span class="activity-heatmap__stat">
                        { format!("{} {} {}", day_counts.len(), active_days_text, in_text) }
                    </span>
                    <span class="activity-heatmap__stat">
                        { format!("{} {}", props.challenge_history.len(), total_chal_text) }
                    </span>
                </div>
            </div>
        </div>
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn count_to_level(count: usize, max_count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    if max_count <= 1 {
        return 4;
    }
    let ratio = count as f64 / max_count as f64;
    if ratio > 0.75 {
        4
    } else if ratio > 0.5 {
        3
    } else if ratio > 0.25 {
        2
    } else {
        1
    }
}

// ─── Preview ─────────────────────────────────────────────────────────────────

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use chrono::{Duration, Utc};
    use konnektoren_core::challenges::{
        Challenge, ChallengeConfig, ChallengeHistory, ChallengeType,
    };
    use yew_preview::prelude::*;
    use yew_preview::test_utils::{exists, has_class, has_text};

    fn create_history_with_pattern(pattern: &[(i64, usize)]) -> ChallengeHistory {
        let mut history = ChallengeHistory::new();
        let mut seq = 0usize;
        for (days_ago, count) in pattern {
            let base = Utc::now() - Duration::days(*days_ago);
            for i in 0..*count {
                // Unique ID + staggered minutes so challenges on the same day
                // are not considered equal by ChallengeHistory::add_challenge.
                let mut challenge = Challenge::new(
                    &ChallengeType::default(),
                    &ChallengeConfig {
                        id: format!("preview-{seq}"),
                        name: format!("Challenge {seq}"),
                        ..ChallengeConfig::default()
                    },
                );
                let offset = Duration::minutes(i as i64 * 15);
                challenge.start_time = Some(base + offset);
                challenge.end_time = Some(base + offset + Duration::minutes(5));
                history.add_challenge(challenge);
                seq += 1;
            }
        }
        history
    }

    fn streak_history() -> ChallengeHistory {
        let mut pattern = Vec::new();
        for day in 0..14i64 {
            pattern.push((day, 3usize));
        }
        create_history_with_pattern(&pattern)
    }

    fn sparse_history() -> ChallengeHistory {
        let pattern: Vec<(i64, usize)> = vec![
            (1, 5),
            (3, 2),
            (7, 8),
            (14, 3),
            (30, 1),
            (60, 4),
            (90, 2),
            (120, 6),
            (180, 3),
            (365, 1),
        ];
        create_history_with_pattern(&pattern)
    }

    fn heavy_history() -> ChallengeHistory {
        let mut pattern = Vec::new();
        for day in 0..30i64 {
            pattern.push((day, ((day % 7) + 1) as usize));
        }
        create_history_with_pattern(&pattern)
    }

    yew_preview::create_preview_with_tests!(
        component: ActivityHeatmapComponent,
        default_props: ActivityHeatmapProps {
            challenge_history: streak_history(),
            days: 365,
        },
        variants: [
            (
                "Sparse Activity",
                ActivityHeatmapProps {
                    challenge_history: sparse_history(),
                    days: 365,
                }
            ),
            (
                "Heavy Learning (30 days)",
                ActivityHeatmapProps {
                    challenge_history: heavy_history(),
                    days: 365,
                }
            ),
            (
                "Empty History",
                ActivityHeatmapProps {
                    challenge_history: ChallengeHistory::new(),
                    days: 365,
                }
            ),
            (
                "Last 90 Days",
                ActivityHeatmapProps {
                    challenge_history: sparse_history(),
                    days: 90,
                }
            ),
        ],
        tests: [
            ("Has activity-heatmap wrapper", has_class("activity-heatmap")),
            ("Shows title", has_text("Activity Heatmap")),
            ("Shows grid cells", exists("activity-heatmap__cell")),
            ("Shows legend", exists("activity-heatmap__legend")),
            ("Shows stats", has_text("active days")),
            ("Shows total challenges", has_text("total challenges")),
        ]
    );
}
