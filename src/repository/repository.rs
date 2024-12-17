use super::repository_error::RepositoryError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Repository<T: Serialize + for<'de> Deserialize<'de>> {
    async fn save(&self, key: &str, value: &T) -> Result<(), RepositoryError>;
    async fn get(&self, key: &str) -> Result<Option<T>, RepositoryError>;
    async fn delete(&self, key: &str) -> Result<(), RepositoryError>;
}
