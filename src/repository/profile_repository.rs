use super::repository::Repository;
use super::repository_error::RepositoryError;
use super::storage::Storage;
use async_trait::async_trait;
use konnektoren_core::prelude::PlayerProfile;

pub const PROFILE_STORAGE_KEY: &str = "konnektoren_profile";

#[async_trait]
pub trait ProfileRepositoryTrait: Send + Sync {
    async fn save_profile(&self, key: &str, profile: &PlayerProfile)
    -> Result<(), RepositoryError>;
    async fn get_profile(&self, key: &str) -> Result<Option<PlayerProfile>, RepositoryError>;
    async fn delete_profile(&self, key: &str) -> Result<(), RepositoryError>;
    async fn update_profile(
        &self,
        key: &str,
        profile: &PlayerProfile,
    ) -> Result<(), RepositoryError>;
}

#[derive(Debug, PartialEq)]
pub struct ProfileRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> ProfileRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync> Repository<PlayerProfile> for ProfileRepository<S> {
    async fn save(&self, key: &str, profile: &PlayerProfile) -> Result<(), RepositoryError> {
        self.storage
            .set(key, profile)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<PlayerProfile>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(profile)) => Ok(Some(profile)),
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
impl<S: Storage + Send + Sync> ProfileRepositoryTrait for ProfileRepository<S> {
    async fn save_profile(
        &self,
        key: &str,
        profile: &PlayerProfile,
    ) -> Result<(), RepositoryError> {
        Repository::save(self, key, profile).await
    }

    async fn get_profile(&self, key: &str) -> Result<Option<PlayerProfile>, RepositoryError> {
        Repository::get(self, key).await
    }

    async fn delete_profile(&self, key: &str) -> Result<(), RepositoryError> {
        Repository::delete(self, key).await
    }

    async fn update_profile(
        &self,
        key: &str,
        profile: &PlayerProfile,
    ) -> Result<(), RepositoryError> {
        self.save_profile(key, profile).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::memory_storage::MemoryStorage;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_profile_repository() {
        let storage = MemoryStorage::default();
        let repo = ProfileRepository::new(storage);
        let key = PROFILE_STORAGE_KEY;

        // Test saving a profile
        let profile = PlayerProfile {
            id: "123".to_string(),
            name: "Alice".to_string(),
            xp: 100,
        };
        repo.update_profile(key, &profile).await.unwrap();

        // Test getting the profile
        let stored_profile = repo.get_profile(key).await.unwrap().unwrap();
        assert_eq!(profile, stored_profile);

        // Test updating the profile
        let updated_profile = PlayerProfile {
            id: "123".to_string(),
            name: "Alice".to_string(),
            xp: 200,
        };
        repo.update_profile(key, &updated_profile).await.unwrap();
        let stored_updated_profile = repo.get_profile(key).await.unwrap().unwrap();
        assert_eq!(updated_profile, stored_updated_profile);

        // Test deleting the profile
        repo.delete_profile(key).await.unwrap();
        let deleted_profile = repo.get_profile(key).await.unwrap();
        assert!(deleted_profile.is_none());
    }
}
