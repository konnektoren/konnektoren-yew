use crate::providers::{use_profile, use_session};
use konnektoren_core::{
    challenges::{Challenge, Performance},
    prelude::{PlayerProfile, Session},
};
use yew::prelude::*;

/// Record a finished attempt in the session and award performance / 10 profile XP.
/// Repository providers persist the resulting state. This does not acknowledge a
/// storage write; apps with explicit save feedback should keep their own adapter.
#[hook]
pub fn use_record_challenge_completion() -> Callback<Challenge> {
    let session = use_session();
    let profile = use_profile();
    let latest = use_mut_ref(|| ((*session).clone(), (*profile).clone()));
    *latest.borrow_mut() = ((*session).clone(), (*profile).clone());

    Callback::from(move |challenge| {
        let mut latest = latest.borrow_mut();
        let (next_session, next_profile) = &mut *latest;
        if record_completion(next_session, next_profile, challenge) {
            session.set(next_session.clone());
            profile.set(next_profile.clone());
        }
    })
}

fn record_completion(
    session: &mut Session,
    profile: &mut PlayerProfile,
    challenge: Challenge,
) -> bool {
    if challenge.end_time.is_none() {
        return false;
    }
    let history = &mut session.game_state.game.challenge_history;
    if history.challenges.iter().any(|previous| {
        previous == &challenge
            || (challenge.start_time.is_some()
                && previous.challenge_config.id == challenge.challenge_config.id
                && previous.start_time == challenge.start_time)
    }) {
        return false;
    }

    let xp = challenge.performance(&challenge.challenge_result) / 10;
    history.add_challenge(challenge.clone());
    profile.xp = profile.xp.saturating_add(xp);
    session.player_profile = profile.clone();
    session.game_state.game.xp = session.game_state.game.xp.saturating_add(xp);
    session.game_state.challenge = challenge;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::challenges::{
        ChallengeConfig, ChallengeResult, ChallengeType, MultipleChoice, MultipleChoiceOption,
        Question, Timed,
    };

    fn attempt() -> Challenge {
        let answer = MultipleChoiceOption {
            id: 0,
            name: "A".into(),
        };
        let mut challenge = Challenge::new(
            &ChallengeType::MultipleChoice(MultipleChoice {
                id: "sample".into(),
                name: "Sample".into(),
                lang: "en".into(),
                options: vec![answer.clone()],
                questions: vec![Question {
                    option: 0,
                    ..Question::default()
                }],
            }),
            &ChallengeConfig {
                id: "sample".into(),
                ..ChallengeConfig::default()
            },
        );
        challenge.start();
        challenge.challenge_result = ChallengeResult::MultipleChoice(vec![answer]);
        challenge.update_end_time();
        challenge
    }

    #[test]
    fn completion_records_results_and_rewards_each_attempt_once() {
        let mut session = Session::default();
        session.game_state.game.challenge_history.challenges.clear();
        let mut profile = PlayerProfile::default();
        let starting_xp = profile.xp;
        let game_xp = session.game_state.game.xp;
        let challenge = attempt();
        assert!(record_completion(
            &mut session,
            &mut profile,
            challenge.clone()
        ));
        assert_eq!(profile.xp, starting_xp + 10);
        assert_eq!(session.game_state.game.xp, game_xp + 10);
        assert_eq!(session.player_profile, profile);
        assert_eq!(session.game_state.challenge, challenge);
        assert_eq!(
            session.game_state.game.challenge_history.challenges,
            vec![challenge.clone()]
        );

        let mut repeated_notification = challenge.clone();
        repeated_notification.end_time = challenge
            .end_time
            .map(|time| time + chrono::Duration::seconds(1));
        assert!(!record_completion(
            &mut session,
            &mut profile,
            repeated_notification
        ));
        assert_eq!(profile.xp, starting_xp + 10);

        let mut retry = challenge;
        retry.start_time = retry
            .start_time
            .map(|time| time + chrono::Duration::seconds(2));
        retry.end_time = retry
            .end_time
            .map(|time| time + chrono::Duration::seconds(3));
        assert!(record_completion(&mut session, &mut profile, retry));
        assert_eq!(profile.xp, starting_xp + 20);
        assert_eq!(session.game_state.game.challenge_history.len(), 2);
        let serialized = serde_json::to_string(&session).expect("session should serialize");
        let restored: Session = serde_json::from_str(&serialized).expect("session should restore");
        assert_eq!(restored, session);
    }

    #[test]
    fn active_challenges_are_not_recorded_or_rewarded() {
        let mut session = Session::default();
        let mut profile = PlayerProfile::default();
        let original = (session.clone(), profile.clone());
        let mut challenge = attempt();
        challenge.end_time = None;
        assert!(!record_completion(&mut session, &mut profile, challenge));
        assert_eq!((session, profile), original);
    }
}
