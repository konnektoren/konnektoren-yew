use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub const BACKUP_KEY: &str = "konnektoren_backup";

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum BackupError {
    #[error("Failed to access backup service: {0}")]
    AccessError(String),

    #[error("Backup not found: {0}")]
    NotFound(String),

    #[error("Unknown error occurred: {0}")]
    Unknown(String),
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Backup<T: Serialize + for<'de> Deserialize<'de>> {
    async fn list_backups(&self) -> Result<Vec<BackupInfo>, BackupError>;
    async fn backup(&self, id: &str, value: &T) -> Result<BackupInfo, BackupError>;
    async fn restore(&self, id: &str) -> Result<T, BackupError>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub name: String,
    pub created_at: String,
}
