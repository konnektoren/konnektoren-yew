use super::storage_error::StorageError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Storage: Clone + PartialEq + Send + Sync + 'static {
    async fn get<T: for<'de> Deserialize<'de> + Sync>(
        &self,
        key: &str,
    ) -> Result<Option<T>, StorageError>;
    async fn set<T: Serialize + Sync>(&self, key: &str, value: &T) -> Result<(), StorageError>;
    async fn remove(&self, key: &str) -> Result<(), StorageError>;
}
