use konnektoren_core::challenges::{ChallengeType, SavedChallenge};
use konnektoren_core::session::Session;

/// Prepares a session when the app starts and after it was loaded. Applications with their
/// own challenge type implement `SessionInitializer<TheirType>`.
pub trait SessionInitializer<C: SavedChallenge = ChallengeType> {
    fn initialize(&self, session: &Session<C>) -> Result<Session<C>, &str>;
}

pub struct DefaultSessionInitializer;

impl SessionInitializer for DefaultSessionInitializer {
    fn initialize(&self, _session: &Session) -> Result<Session, &str> {
        Ok(Session::default())
    }
}
