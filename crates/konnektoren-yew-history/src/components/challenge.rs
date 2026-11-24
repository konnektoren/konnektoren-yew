//! The result badge of a challenge attempt.
use super::EntryResult;
use crate::model::challenge::{band_tone, challenge_score};
use konnektoren_core::challenges::{Challenge, PerformanceScale};

/// The attempt's result as a percentage badge, coloured by its band on `scale`.
pub fn challenge_result(challenge: &Challenge, scale: &PerformanceScale) -> EntryResult {
    let score = challenge_score(challenge);
    EntryResult::new(format!("{score}%")).with_tone(band_tone(scale.band(score)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{
        DefaultHistoryEntry, EntryContext, History, HistoryContext, HistoryView,
    };
    use crate::model::challenge::{ChallengeHistoryPage, ChallengeSort, Results};
    use crate::model::{
        Direction, FilterContext, HistoryFilters, HistoryItem, HistoryLabels, HistoryOption,
        SortKey, Tone, select,
    };
    use chrono::Utc;
    use konnektoren_core::challenges::PerformanceScale;
    use konnektoren_core::challenges::{
        ChallengeConfig, ChallengeResult, ChallengeType, MultipleChoice, MultipleChoiceOption,
        Question,
    };
    use yew::prelude::*;

    /// An attempt scoring `correct` of 10.
    fn attempt(name: &str, correct: usize, day: u32) -> Challenge {
        let content = ChallengeType::MultipleChoice(MultipleChoice {
            id: "sample".into(),
            name: "Sample".into(),
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
                id: name.to_lowercase(),
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
        challenge.end_time =
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2026, 11, day, 12, 0, 0).single();
        challenge
    }

    #[derive(Properties, PartialEq)]
    struct Props {
        attempts: Vec<Challenge>,
    }

    /// A page on the default scale.
    struct Attempts;

    impl History for Attempts {
        type Item = Challenge;
        type Filters = (Results,);
        type Sort = ChallengeSort;
        type Properties = Props;
        type Message = ();
        fn create(_: &HistoryContext<'_, Self>) -> Self {
            Self
        }
        fn items(props: &Props) -> Result<Vec<Challenge>, AttrValue> {
            Ok(props.attempts.clone())
        }
        fn labels() -> HistoryLabels {
            HistoryLabels::default()
        }
    }

    impl ChallengeHistoryPage for Attempts {}

    impl DefaultHistoryEntry<Attempts> for Challenge {
        fn result(&self, _: &EntryContext<'_, Attempts>) -> Option<EntryResult> {
            Some(challenge_result(self, &Attempts::SCALE))
        }
    }

    /// A page of an app that passes from 50 %.
    struct Lenient;

    impl History for Lenient {
        type Item = Challenge;
        type Filters = (Results,);
        type Sort = ChallengeSort;
        type Properties = Props;
        type Message = ();
        fn create(_: &HistoryContext<'_, Self>) -> Self {
            Self
        }
        fn items(props: &Props) -> Result<Vec<Challenge>, AttrValue> {
            Ok(props.attempts.clone())
        }
        fn labels() -> HistoryLabels {
            HistoryLabels::default()
        }
    }

    impl DefaultHistoryEntry<Lenient> for Challenge {}

    impl ChallengeHistoryPage for Lenient {
        const SCALE: PerformanceScale = PerformanceScale::DEFAULT.with_passed(50);
    }

    fn listed<H: ChallengeHistoryPage>(
        history: &H,
        props: &H::Properties,
        results: Results,
        attempts: &[Challenge],
    ) -> Vec<u32> {
        let ctx = FilterContext { props, history };
        let filters = (results,);
        select(
            attempts,
            "",
            |c| c.name(),
            |c| HistoryFilters::<H>::admits(&filters, c, &ctx),
            |_| SortKey::Base,
            Direction::Descending,
        )
        .into_iter()
        .map(|entry| challenge_score(entry.item))
        .collect()
    }

    #[test]
    fn results_filter_by_band_on_the_page_scale() {
        let attempts = vec![
            attempt("A", 9, 1),
            attempt("B", 7, 2),
            attempt("C", 6, 3),
            attempt("D", 5, 4),
        ];
        let props = Props {
            attempts: attempts.clone(),
        };
        assert_eq!(
            listed(&Attempts, &props, Results::Mastered, &attempts),
            [90]
        );
        assert_eq!(listed(&Attempts, &props, Results::Good, &attempts), [70]);
        assert_eq!(listed(&Attempts, &props, Results::Passed, &attempts), [60]);
        assert_eq!(
            listed(&Attempts, &props, Results::NotPassed, &attempts),
            [50]
        );
        assert_eq!(
            listed(&Attempts, &props, Results::All, &attempts),
            [50, 60, 70, 90]
        );
        // 50 % passes when the app passes from 50 %.
        assert_eq!(
            listed(&Lenient, &props, Results::NotPassed, &attempts),
            Vec::<u32>::new()
        );
        assert_eq!(
            listed(&Lenient, &props, Results::Passed, &attempts),
            [50, 60]
        );
    }

    #[test]
    fn options_round_trip_and_badges_follow_the_bands() {
        for option in Results::ALL {
            assert_eq!(Results::parse(option.value()), *option);
        }
        let scale = PerformanceScale::DEFAULT;
        assert_eq!(
            challenge_result(&attempt("A", 8, 1), &scale).tone,
            Tone::Good
        );
        assert_eq!(
            challenge_result(&attempt("A", 7, 1), &scale).tone,
            Tone::Info
        );
        assert_eq!(
            challenge_result(&attempt("A", 6, 1), &scale).tone,
            Tone::Neutral
        );
        assert_eq!(
            challenge_result(&attempt("A", 5, 1), &scale).tone,
            Tone::Warn
        );
        assert_eq!(challenge_result(&attempt("A", 5, 1), &scale).text, "50%");
    }

    #[tokio::test]
    async fn renders_challenge_attempts_with_band_filter_and_badges() {
        let html = yew::ServerRenderer::<HistoryView<Attempts>>::with_props(|| Props {
            attempts: vec![attempt("Ode to Joy", 9, 1), attempt("Silent Night", 5, 2)],
        })
        .render()
        .await;
        assert!(html.contains("Ode to Joy"));
        assert!(html.contains("pill--good"));
        assert!(html.contains("pill--warn"));
        assert!(html.contains("value=\"not-passed\""));
        assert!(html.contains("Mastered"));
    }
}
