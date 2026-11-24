//! An example history of challenge attempts, judged on core's
//! [`PerformanceScale`]: band filter, sort orders, summary and result badges.
use crate::challenge::{
    ChallengeHistoryPage, ChallengeSort, Results, band_label, band_tone, challenge_result,
    challenge_score,
};
use crate::{
    DefaultHistoryEntry, EntryContext, EntryMeta, EntryResult, History, HistoryContext,
    HistoryLabels,
};
use chrono::{TimeZone, Utc};
use konnektoren_core::challenges::{
    Challenge, ChallengeConfig, ChallengeResult, ChallengeType, MultipleChoice,
    MultipleChoiceOption, PerformanceScale, Question,
};
use konnektoren_yew_core::{Stat, StatGrid};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SampleAttemptsProps {
    pub attempts: Vec<Challenge>,
}

/// Exercise attempts on the default scale (passed 60, good 70, mastered 80).
pub struct SampleAttempts;

impl History for SampleAttempts {
    type Item = Challenge;
    type Filters = (Results,);
    type Sort = ChallengeSort;
    type Properties = SampleAttemptsProps;
    type Message = ();

    const PAGE: usize = 8;

    fn create(_: &HistoryContext<'_, Self>) -> Self {
        Self
    }

    fn items(props: &SampleAttemptsProps) -> Result<Vec<Challenge>, AttrValue> {
        Ok(props.attempts.clone())
    }

    fn labels() -> HistoryLabels {
        HistoryLabels {
            title: "Exercise history",
            intro: "Every exercise you completed, judged on the app's scale.",
            search_label: "Search exercises",
            search_placeholder: "Exercise name",
            noun: "Attempts",
            empty: "No challenges completed yet.",
            no_match: "No attempts match these filters.",
        }
    }

    fn view_summary(&self, ctx: &HistoryContext<'_, Self>, attempts: &[Challenge]) -> Html {
        let scale = Self::SCALE;
        let scores: Vec<u32> = attempts.iter().map(challenge_score).collect();
        let count = |band: fn(&PerformanceScale, u32) -> bool| {
            scores
                .iter()
                .filter(|score| band(&scale, **score))
                .count()
                .to_string()
        };
        let tiles = vec![
            Stat::new(ctx.t("Attempts"), attempts.len().to_string())
                .with_icon(html! { "📝" })
                .highlighted(),
            Stat::new(ctx.t("Passed"), count(PerformanceScale::is_passed))
                .with_icon(html! { "✅" }),
            Stat::new(ctx.t("Mastered"), count(PerformanceScale::is_mastered))
                .with_icon(html! { "🏆" }),
            Stat::new(
                ctx.t("Best result"),
                scores
                    .iter()
                    .max()
                    .map_or("—".into(), |best| format!("{best}%")),
            )
            .with_icon(html! { "⭐" }),
        ];
        html! { <StatGrid stats={tiles} /> }
    }
}

impl ChallengeHistoryPage for SampleAttempts {}

impl DefaultHistoryEntry<SampleAttempts> for Challenge {
    fn meta(&self, ctx: &EntryContext<'_, SampleAttempts>) -> Vec<EntryMeta> {
        let band = SampleAttempts::SCALE.band(challenge_score(self));
        let stars = "★".repeat(band.stars() as usize) + &"☆".repeat(3 - band.stars() as usize);
        vec![
            EntryMeta::text(ctx.t(band_label(band))).with_tone(band_tone(band)),
            EntryMeta::text(stars).with_title(ctx.t("Stars")),
        ]
    }

    fn result(&self, ctx: &EntryContext<'_, SampleAttempts>) -> Option<EntryResult> {
        Some(challenge_result(self, &SampleAttempts::SCALE).with_label(ctx.t("Result")))
    }
}

const EXERCISES: [&str; 5] = [
    "Articles",
    "Prepositions",
    "Verb endings",
    "Word order",
    "Adjective endings",
];

/// An attempt of `name` scoring `correct` of 10, on `day` of November 2026.
pub fn sample_attempt(name: &str, correct: usize, day: u32) -> Challenge {
    let content = ChallengeType::MultipleChoice(MultipleChoice {
        id: "sample".into(),
        name: "Multiple choice".into(),
        lang: "en".into(),
        options: (0..2)
            .map(|id| MultipleChoiceOption {
                id,
                name: id.to_string(),
            })
            .collect(),
        questions: (0..10)
            .map(|_| Question {
                option: 0,
                ..Question::default()
            })
            .collect(),
    });
    let mut challenge = Challenge::new(
        &content,
        &ChallengeConfig {
            id: name.to_lowercase().replace(' ', "-"),
            name: name.into(),
            ..ChallengeConfig::default()
        },
    );
    challenge.challenge_result = Some(ChallengeResult::MultipleChoice(
        (0..10)
            .map(|answer| MultipleChoiceOption {
                id: usize::from(answer >= correct),
                name: "Answer".into(),
            })
            .collect(),
    ));
    challenge.end_time = Utc.with_ymd_and_hms(2026, 11, day, 12, 0, 0).single();
    challenge
}

/// `count` attempts over several exercises, covering every band.
pub fn sample_attempts(count: usize) -> Vec<Challenge> {
    (0..count)
        .map(|index| {
            sample_attempt(
                EXERCISES[index % EXERCISES.len()],
                (index * 3 + 4) % 7 + 4,
                1 + (index as u32) % 28,
            )
        })
        .collect()
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::{HistoryPreview, HistoryPreviewProps, HistoryView};
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        HistoryView<SampleAttempts>,
        SampleAttemptsProps {
            attempts: sample_attempts(14)
        },
        (
            "A few attempts",
            SampleAttemptsProps {
                attempts: sample_attempts(3)
            }
        ),
        (
            "No attempts",
            SampleAttemptsProps {
                attempts: Vec::new()
            }
        ),
    );

    yew_preview::create_preview!(
        HistoryPreview<SampleAttempts>,
        HistoryPreviewProps {
            data: SampleAttemptsProps {
                attempts: sample_attempts(10),
            },
            limit: 5,
            more: html! {
                <a href="#" class="btn btn-ghost btn-sm">
                    <i class="fa-solid fa-clock-rotate-left" aria-hidden="true"></i>{"Full history"}
                </a>
            },
            title: None,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HistoryView;
    use konnektoren_core::challenges::PerformanceBand;

    #[test]
    fn sample_attempts_cover_every_band() {
        let bands: Vec<PerformanceBand> = sample_attempts(14)
            .iter()
            .map(|attempt| SampleAttempts::SCALE.band(challenge_score(attempt)))
            .collect();
        for band in [
            PerformanceBand::NotPassed,
            PerformanceBand::Passed,
            PerformanceBand::Good,
            PerformanceBand::Mastered,
        ] {
            assert!(bands.contains(&band), "{band:?} missing");
        }
    }

    #[tokio::test]
    async fn renders_the_challenge_sample() {
        let html = yew::ServerRenderer::<HistoryView<SampleAttempts>>::with_props(|| {
            SampleAttemptsProps {
                attempts: sample_attempts(14),
            }
        })
        .render()
        .await;
        assert!(html.contains("Exercise history"));
        assert!(html.contains("stat-grid"));
        assert!(html.contains("★"));
        assert!(html.contains("pill--good"));
    }
}
