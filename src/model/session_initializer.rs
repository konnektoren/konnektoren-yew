use konnektoren_core::challenges::{ChallengeType, SavedChallenge};
use konnektoren_core::session::Session;

/// Prepares a session when the app starts and after it was loaded. Applications with their
/// own challenge type implement `SessionInitializer<TheirType>`.
pub trait SessionInitializer<C: SavedChallenge = ChallengeType> {
    fn initialize(&self, session: &Session<C>) -> Result<Session<C>, &str>;
}

pub struct DefaultSessionInitializer;

impl SessionInitializer for DefaultSessionInitializer {
    fn initialize(&self, session: &Session) -> Result<Session, &str> {
        Ok(session.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_loaded_progress() {
        let mut saved = Session::default();
        saved.game_state.game.xp = 42;
        let restored = DefaultSessionInitializer.initialize(&saved).unwrap();
        assert_eq!(restored, saved);
    }
}
