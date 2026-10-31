use super::repository::Repository;
use super::repository_error::RepositoryError;
use super::storage::Storage;
use async_trait::async_trait;
use konnektoren_core::challenges::{ChallengeType, SavedChallenge};
use konnektoren_core::session::Session;

pub const SESSION_STORAGE_KEY: &str = "konnektoren_session";

/// Stores sessions. Applications with their own challenge type use
/// `SessionRepositoryTrait<TheirType>`.
#[async_trait]
pub trait SessionRepositoryTrait<C: SavedChallenge = ChallengeType>: Send + Sync {
    async fn save_session(&self, key: &str, session: &Session<C>) -> Result<(), RepositoryError>;
    async fn get_session(&self, key: &str) -> Result<Option<Session<C>>, RepositoryError>;
    async fn delete_session(&self, key: &str) -> Result<(), RepositoryError>;
    async fn update_session(&self, key: &str, session: &Session<C>) -> Result<(), RepositoryError>;
}

#[derive(Debug, PartialEq)]
pub struct SessionRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> SessionRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync, C: SavedChallenge> Repository<Session<C>> for SessionRepository<S> {
    async fn save(&self, key: &str, session: &Session<C>) -> Result<(), RepositoryError> {
        self.storage
            .set(key, session)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<Session<C>>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(session)) => Ok(Some(session)),
            Ok(None) => Ok(None),
            Err(e) => Err(RepositoryError::StorageError(e.to_string())),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), RepositoryError> {
        self.storage
            .remove(key)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }
}

#[async_trait]
impl<S: Storage + Send + Sync, C: SavedChallenge> SessionRepositoryTrait<C>
    for SessionRepository<S>
{
    async fn save_session(&self, key: &str, session: &Session<C>) -> Result<(), RepositoryError> {
        Repository::<Session<C>>::save(self, key, session).await
    }

    async fn get_session(&self, key: &str) -> Result<Option<Session<C>>, RepositoryError> {
        Repository::<Session<C>>::get(self, key).await
    }

    async fn delete_session(&self, key: &str) -> Result<(), RepositoryError> {
        Repository::<Session<C>>::delete(self, key).await
    }

    async fn update_session(&self, key: &str, session: &Session<C>) -> Result<(), RepositoryError> {
        self.save_session(key, session).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::memory_storage::MemoryStorage;
    use konnektoren_core::game::GameState;
    use konnektoren_core::prelude::PlayerProfile;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_session_repository() {
        let storage = MemoryStorage::default();
        let repo = SessionRepository::new(storage);
        let key = SESSION_STORAGE_KEY;

        // Test saving a session
        let session = Session {
            id: "123".to_string(),
            player_profile: PlayerProfile {
                id: "123".to_string(),
                name: "player123".to_string(),
                xp: 0,
            },
            game_state: GameState::default(),
        };
        repo.update_session(key, &session).await.unwrap();

        // Test getting the session
        let stored_session = repo.get_session(key).await.unwrap().unwrap();
        assert_eq!(session, stored_session);

        // Test updating the session
        let updated_session = Session {
            id: "123".to_string(),
            player_profile: PlayerProfile {
                id: "123".to_string(),
                name: "player123".to_string(),
                xp: 0,
            },
            game_state: GameState::default(),
        };
        repo.update_session(key, &updated_session).await.unwrap();
        let stored_updated_session = repo.get_session(key).await.unwrap().unwrap();
        assert_eq!(updated_session, stored_updated_session);

        // Test deleting the session
        SessionRepositoryTrait::<ChallengeType>::delete_session(&repo, key)
            .await
            .unwrap();
        let deleted_session: Option<Session> = repo.get_session(key).await.unwrap();
        assert!(deleted_session.is_none());
    }
}
