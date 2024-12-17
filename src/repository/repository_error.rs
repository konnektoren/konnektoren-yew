use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Failed to serialize or deserialize data: {0}")]
    SerializationError(#[from] SerdeError),

    #[error("Storage access error: {0}")]
    StorageError(String),

    #[error("Data not found")]
    NotFound,

    #[error("Unknown error occurred")]
    Unknown,
}
