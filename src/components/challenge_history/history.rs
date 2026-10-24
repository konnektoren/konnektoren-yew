use konnektoren_core::challenges::{Challenge, ChallengeHistory};
use std::cmp::Reverse;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeHistoryProps {
    pub challenge_history: ChallengeHistory,
    /// Match the configured challenge ID, rather than its shared exercise type.
    #[prop_or_default]
    pub challenge_id: Option<AttrValue>,
    /// Number of recent attempts to display. Counts still describe the full history.
    #[prop_or_default]
    pub limit: Option<usize>,
    /// Optional selection action, for example reviewing an attempt or opening its challenge history.
    #[prop_or_default]
    pub on_select: Option<Callback<Challenge>>,
}

pub(super) fn entries<'a>(
    history: &'a ChallengeHistory,
    challenge_id: Option<&str>,
    limit: Option<usize>,
) -> Vec<(usize, &'a Challenge)> {
    let mut entries: Vec<_> = history
        .challenges
        .iter()
        .enumerate()
        .filter(|(_, challenge)| challenge_id.is_none_or(|id| challenge.challenge_config.id == id))
        .collect();
    // Timestamps also order merged histories. For undated/tied attempts, prefer later additions.
    entries.sort_by_key(|(index, challenge)| {
        Reverse((challenge.end_time.or(challenge.start_time), *index))
    });
    if let Some(limit) = limit {
        entries.truncate(limit);
    }
    entries
}

pub(super) fn name(challenge: &Challenge) -> String {
    if challenge.challenge_config.name.trim().is_empty() {
        challenge.challenge_type.name().to_string()
    } else {
        challenge.challenge_config.name.clone()
    }
}

pub(super) fn date(challenge: &Challenge) -> Html {
    challenge.end_time.or(challenge.start_time).map(|date| html! {
        <time datetime={date.to_rfc3339()}>{date.format("%Y-%m-%d %H:%M UTC").to_string()}</time>
    }).unwrap_or_else(|| html! { <span>{"—"}</span> })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use konnektoren_core::challenges::ChallengeConfig;

    fn attempt(id: &str, timestamp: Option<i64>) -> Challenge {
        Challenge {
            challenge_config: ChallengeConfig {
                id: id.into(),
                ..ChallengeConfig::default()
            },
            end_time: timestamp.and_then(|timestamp| Utc.timestamp_opt(timestamp, 0).single()),
            ..Challenge::default()
        }
    }

    #[test]
    fn filters_by_configured_id_before_limiting() {
        let history = ChallengeHistory {
            challenges: vec![
                attempt("a", Some(1)),
                attempt("b", Some(3)),
                attempt("a", Some(2)),
            ],
        };
        let selected = entries(&history, Some("a"), Some(1));
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].0, 2);
        assert!(entries(&history, Some("missing"), None).is_empty());
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn orders_merged_history_by_time_and_undated_attempts_by_insertion() {
        let history = ChallengeHistory {
            challenges: vec![
                attempt("a", Some(3)),
                attempt("a", None),
                attempt("a", Some(1)),
                attempt("a", None),
                attempt("a", Some(3)),
            ],
        };
        let indices: Vec<_> = entries(&history, None, None)
            .into_iter()
            .map(|(index, _)| index)
            .collect();
        assert_eq!(indices, vec![4, 0, 2, 3, 1]);
        assert!(entries(&history, None, Some(0)).is_empty());
    }
}
