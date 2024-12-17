use super::{SessionRepositoryTrait, SESSION_STORAGE_KEY};
use anyhow::Result;
use konnektoren_core::game::GameState;
use konnektoren_core::persistence::GameStatePersistence;
use std::sync::{Arc, RwLock};

pub struct GameStatePersistenceImpl {
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session: Arc<RwLock<konnektoren_core::session::Session>>,
}

impl GameStatePersistence for GameStatePersistenceImpl {
    fn save_game_state(&self, state: &GameState) -> Result<()> {
        let session_repository = self.session_repository.clone();
        let session = self.session.clone();
        let state = state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let mut session_guard = session.write().unwrap();
            session_guard.game_state = state;
            session_repository
                .save_session(SESSION_STORAGE_KEY, &session_guard)
                .await
                .unwrap();
        });
        Ok(())
    }

    fn load_game_state(&self) -> Result<GameState> {
        let session_guard = self.session.read().unwrap();
        Ok(session_guard.game_state.clone())
    }
}
