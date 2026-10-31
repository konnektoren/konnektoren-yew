use super::{SESSION_STORAGE_KEY, SessionRepositoryTrait};
use konnektoren_core::challenges::{Attempt, ChallengeHistory, ChallengeType, SavedChallenge};
use konnektoren_core::game::GameState;
use konnektoren_core::persistence::{GameStatePersistence, PersistenceError, Result};
use konnektoren_core::session::Session;
use std::sync::{Arc, RwLock};

/// Saves the controller's game state into the player's session. Applications with their own
/// challenge type use `GameStatePersistenceImpl<TheirType>`.
pub struct GameStatePersistenceImpl<C: SavedChallenge = ChallengeType> {
    pub session_repository: Arc<dyn SessionRepositoryTrait<C>>,
    pub session: Arc<RwLock<Session<C>>>,
}

impl<C: SavedChallenge> GameStatePersistenceImpl<C> {
    pub fn new(
        session_repository: Arc<dyn SessionRepositoryTrait<C>>,
        session: Arc<RwLock<Session<C>>>,
    ) -> Self {
        Self {
            session_repository,
            session,
        }
    }

    /// Check if this exact challenge instance is already in history
    fn is_challenge_in_history(
        challenge_history: &ChallengeHistory<Attempt<C>>,
        challenge: &Attempt<C>,
    ) -> bool {
        challenge_history.challenges.iter().any(|c| {
            c.challenge_config.id == challenge.challenge_config.id
                && c.start_time == challenge.start_time
                && c.end_time == challenge.end_time
        })
    }
}

impl<C: SavedChallenge> GameStatePersistence<C> for GameStatePersistenceImpl<C> {
    fn save_game_state(&self, state: &GameState<C>) -> Result<()> {
        #[cfg(feature = "csr")]
        {
            use wasm_bindgen_futures::spawn_local;

            let session_repository = self.session_repository.clone();
            let session = self.session.clone();
            let state = state.clone();

            spawn_local(async move {
                let mut session_guard = match session.write() {
                    Ok(guard) => guard,
                    Err(e) => {
                        tracing::error!("Failed to acquire write lock on session: {}", e);
                        return;
                    }
                };

                session_guard.game_state = state.clone();

                // Check if challenge is completed and not already in history
                if state.challenge.start_time.is_some() && state.challenge.end_time.is_some() {
                    let is_already_saved = Self::is_challenge_in_history(
                        &session_guard.game_state.game.challenge_history,
                        &state.challenge,
                    );

                    if !is_already_saved {
                        tracing::info!(
                            "Adding completed challenge to history: {}",
                            state.challenge.challenge_config.id
                        );

                        session_guard
                            .game_state
                            .game
                            .challenge_history
                            .add_challenge(state.challenge.clone());
                    } else {
                        tracing::debug!(
                            "Challenge already in history, skipping: {}",
                            state.challenge.challenge_config.id
                        );
                    }
                }

                if let Err(e) = session_repository
                    .save_session(SESSION_STORAGE_KEY, &session_guard)
                    .await
                {
                    tracing::error!("Failed to save session: {:?}", e);
                }
            });
        }
        Ok(())
    }

    fn load_game_state(&self) -> Result<GameState<C>> {
        let session_guard = self.session.read().map_err(|e| {
            PersistenceError::AccessError(format!("Failed to acquire read lock on session: {}", e))
        })?;
        Ok(session_guard.game_state.clone())
    }
}
