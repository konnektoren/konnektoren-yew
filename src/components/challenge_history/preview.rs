use super::*;
use chrono::{TimeZone, Utc};
use konnektoren_core::challenges::{
    Challenge, ChallengeConfig, ChallengeHistory, ChallengeResult, ChallengeType, MultipleChoice,
    MultipleChoiceOption, Question,
};
use yew::prelude::*;
use yew_preview::prelude::*;

fn sample_history() -> ChallengeHistory {
    let content = ChallengeType::MultipleChoice(MultipleChoice {
        id: "sample".into(),
        name: "Sample exercise".into(),
        lang: "en".into(),
        options: vec![
            MultipleChoiceOption {
                id: 0,
                name: "A".into(),
            },
            MultipleChoiceOption {
                id: 1,
                name: "B".into(),
            },
        ],
        questions: (0..4)
            .map(|_| Question {
                option: 0,
                ..Question::default()
            })
            .collect(),
    });
    let mut history = ChallengeHistory::new();
    for (index, (id, name, correct)) in [
        ("exercise-1", "First exercise", 1),
        ("exercise-2", "Second exercise", 2),
        ("exercise-1", "First exercise", 3),
        ("exercise-1", "First exercise", 4),
    ]
    .into_iter()
    .enumerate()
    {
        let mut challenge = Challenge::new(
            &content,
            &ChallengeConfig {
                id: id.into(),
                name: name.into(),
                ..ChallengeConfig::default()
            },
        );
        challenge.challenge_result = ChallengeResult::MultipleChoice(
            (0..4)
                .map(|answer| MultipleChoiceOption {
                    id: usize::from(answer >= correct),
                    name: "Answer".into(),
                })
                .collect(),
        );
        challenge.start_time = Utc
            .with_ymd_and_hms(2026, 10, 20 + index as u32, 10, 0, 0)
            .single();
        challenge.end_time = challenge
            .start_time
            .map(|date| date + chrono::Duration::seconds(45));
        history.add_challenge(challenge);
    }
    history
}

fn props() -> ChallengeHistoryProps {
    ChallengeHistoryProps {
        challenge_history: sample_history(),
        challenge_id: None,
        limit: None,
        on_select: None,
    }
}

yew_preview::create_preview!(
    ChallengeHistorySummaryComponent,
    props(),
    (
        "empty",
        ChallengeHistoryProps {
            challenge_history: ChallengeHistory::new(),
            ..props()
        }
    ),
);
yew_preview::create_preview!(
    ChallengeHistoryTable,
    props(),
    (
        "recent two",
        ChallengeHistoryProps {
            limit: Some(2),
            ..props()
        }
    ),
    (
        "missing dates",
        ChallengeHistoryProps {
            challenge_history: {
                let mut history = sample_history();
                for challenge in &mut history.challenges {
                    challenge.start_time = None;
                    challenge.end_time = None;
                }
                history
            },
            ..props()
        }
    ),
);
yew_preview::create_preview!(
    ChallengeHistoryList,
    props(),
    (
        "recent two",
        ChallengeHistoryProps {
            limit: Some(2),
            ..props()
        }
    ),
    (
        "one challenge",
        ChallengeHistoryProps {
            challenge_id: Some("exercise-1".into()),
            ..props()
        }
    ),
    (
        "empty",
        ChallengeHistoryProps {
            challenge_history: ChallengeHistory::new(),
            ..props()
        }
    ),
);
yew_preview::create_preview!(
    ChallengeAttemptHistory,
    ChallengeAttemptHistoryProps {
        challenge_history: sample_history(),
        challenge_id: "exercise-1".into(),
        limit: None,
        on_select: None
    },
    (
        "recent two",
        ChallengeAttemptHistoryProps {
            challenge_history: sample_history(),
            challenge_id: "exercise-1".into(),
            limit: Some(2),
            on_select: None
        }
    ),
    (
        "no attempts",
        ChallengeAttemptHistoryProps {
            challenge_history: sample_history(),
            challenge_id: "not-attempted".into(),
            limit: None,
            on_select: None
        }
    ),
);
