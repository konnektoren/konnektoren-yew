#[cfg(feature = "backup")]
mod backup;
mod certificate_repository;
mod game_state_persistence_impl;
#[cfg(feature = "gdrive")]
mod gdrive_backup;
mod inbox_repository;
mod profile_repository;
mod session_repository;
mod settings_repository;

mod local_storage;
mod memory_storage;

mod repository;
mod repository_error;

mod storage;
mod storage_error;

#[cfg(feature = "backup")]
pub use backup::{BACKUP_KEY, Backup, BackupError, BackupInfo};
pub use certificate_repository::{
    CERTIFICATE_STORAGE_KEY, CertificateRepository, CertificateRepositoryTrait,
};
pub use game_state_persistence_impl::GameStatePersistenceImpl;
pub use inbox_repository::{INBOX_STORAGE_KEY, InboxRepository, InboxRepositoryTrait};
pub use profile_repository::{PROFILE_STORAGE_KEY, ProfileRepository, ProfileRepositoryTrait};
pub use session_repository::{SESSION_STORAGE_KEY, SessionRepository, SessionRepositoryTrait};
pub use settings_repository::{SETTINGS_STORAGE_KEY, SettingsRepository, SettingsRepositoryTrait};

pub use local_storage::LocalStorage;
pub use memory_storage::MemoryStorage;

pub use repository::Repository;
pub use repository_error::RepositoryError;

#[cfg(feature = "gdrive")]
pub use gdrive_backup::GDriveBackup;
pub use storage::Storage;
pub use storage_error::StorageError;
