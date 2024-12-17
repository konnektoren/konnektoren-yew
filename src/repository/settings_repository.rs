use super::repository::Repository;
use super::repository_error::RepositoryError;
use super::storage::Storage;
use crate::model::Settings;
use async_trait::async_trait;

pub const SETTINGS_STORAGE_KEY: &str = "konnektoren_settings";

#[async_trait]
pub trait SettingsRepositoryTrait: Send + Sync {
    async fn save_settings(&self, key: &str, settings: &Settings) -> Result<(), RepositoryError>;
    async fn get_settings(&self, key: &str) -> Result<Option<Settings>, RepositoryError>;
    async fn delete_settings(&self, key: &str) -> Result<(), RepositoryError>;
}

#[derive(Debug, PartialEq)]
pub struct SettingsRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> SettingsRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync> Repository<Settings> for SettingsRepository<S> {
    async fn save(&self, key: &str, settings: &Settings) -> Result<(), RepositoryError> {
        self.storage
            .set(key, settings)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<Settings>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(settings)) => Ok(Some(settings)),
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
impl<S: Storage + Send + Sync> SettingsRepositoryTrait for SettingsRepository<S> {
    async fn save_settings(&self, key: &str, settings: &Settings) -> Result<(), RepositoryError> {
        Repository::save(self, key, settings).await
    }

    async fn get_settings(&self, key: &str) -> Result<Option<Settings>, RepositoryError> {
        Repository::get(self, key).await
    }

    async fn delete_settings(&self, key: &str) -> Result<(), RepositoryError> {
        Repository::delete(self, key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Settings;
    use crate::repository::MemoryStorage;

    #[tokio::test]
    async fn test_save_and_get() {
        let storage = MemoryStorage::default();
        let repository = SettingsRepository::new(storage);

        let settings = Settings::default();
        repository
            .save_settings(SETTINGS_STORAGE_KEY, &settings)
            .await
            .unwrap();

        let loaded = repository
            .get_settings(SETTINGS_STORAGE_KEY)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(settings, loaded);
    }

    #[tokio::test]
    async fn test_delete() {
        let storage = MemoryStorage::default();
        let repository = SettingsRepository::new(storage);

        let settings = Settings::default();
        repository
            .save_settings(SETTINGS_STORAGE_KEY, &settings)
            .await
            .unwrap();

        repository
            .delete_settings(SETTINGS_STORAGE_KEY)
            .await
            .unwrap();

        let loaded = repository.get_settings(SETTINGS_STORAGE_KEY).await.unwrap();
        assert!(loaded.is_none());
    }
}
