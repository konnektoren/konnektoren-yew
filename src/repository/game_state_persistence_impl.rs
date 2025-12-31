use super::{SESSION_STORAGE_KEY, SessionRepositoryTrait};
use konnektoren_core::game::GameState;
use konnektoren_core::persistence::{GameStatePersistence, PersistenceError, Result};
use std::sync::{Arc, RwLock};

pub struct GameStatePersistenceImpl {
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session: Arc<RwLock<konnektoren_core::session::Session>>,
}

impl GameStatePersistenceImpl {
    pub fn new(
        session_repository: Arc<dyn SessionRepositoryTrait>,
        session: Arc<RwLock<konnektoren_core::session::Session>>,
    ) -> Self {
        Self {
            session_repository,
            session,
        }
    }

    /// Check if this exact challenge instance is already in history
    fn is_challenge_in_history(
        challenge_history: &konnektoren_core::challenges::ChallengeHistory,
        challenge: &konnektoren_core::challenges::Challenge,
    ) -> bool {
        challenge_history.challenges.iter().any(|c| {
            c.challenge_config.id == challenge.challenge_config.id
                && c.start_time == challenge.start_time
                && c.end_time == challenge.end_time
        })
    }
}

impl GameStatePersistence for GameStatePersistenceImpl {
    fn save_game_state(&self, state: &GameState) -> Result<()> {
        let session_repository = self.session_repository.clone();
        let session = self.session.clone();
        let state = state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let mut session_guard = match session.write() {
                Ok(guard) => guard,
                Err(e) => {
                    log::error!("Failed to acquire write lock on session: {}", e);
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
                    log::info!(
                        "Adding completed challenge to history: {}",
                        state.challenge.challenge_config.id
                    );

                    session_guard
                        .game_state
                        .game
                        .challenge_history
                        .add_challenge(state.challenge.clone());
                } else {
                    log::debug!(
                        "Challenge already in history, skipping: {}",
                        state.challenge.challenge_config.id
                    );
                }
            }

            if let Err(e) = session_repository
                .save_session(SESSION_STORAGE_KEY, &session_guard)
                .await
            {
                log::error!("Failed to save session: {:?}", e);
            }
        });
        Ok(())
    }

    fn load_game_state(&self) -> Result<GameState> {
        let session_guard = self.session.read().map_err(|e| {
            PersistenceError::AccessError(format!("Failed to acquire read lock on session: {}", e))
        })?;
        Ok(session_guard.game_state.clone())
    }
}
