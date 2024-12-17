use super::repository::Repository;
use super::repository_error::RepositoryError;
use super::storage::Storage;
use async_trait::async_trait;
use konnektoren_core::certificates::CertificateData;

pub const CERTIFICATE_STORAGE_KEY: &str = "konnektoren_certificates";

#[async_trait]
pub trait CertificateRepositoryTrait: Send + Sync {
    async fn save_certificates(
        &self,
        key: &str,
        certificates: &Vec<CertificateData>,
    ) -> Result<(), RepositoryError>;
    async fn get_certificates(
        &self,
        key: &str,
    ) -> Result<Option<Vec<CertificateData>>, RepositoryError>;
    async fn delete_certificates(&self, key: &str) -> Result<(), RepositoryError>;
    async fn add_certificate(
        &self,
        key: &str,
        certificate: CertificateData,
    ) -> Result<(), RepositoryError>;
    async fn list_certificates(&self, key: &str) -> Result<Vec<CertificateData>, RepositoryError>;
}

#[derive(Debug, PartialEq)]
pub struct CertificateRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> CertificateRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync> Repository<Vec<CertificateData>> for CertificateRepository<S> {
    async fn save(
        &self,
        key: &str,
        certificates: &Vec<CertificateData>,
    ) -> Result<(), RepositoryError> {
        self.storage
            .set(key, certificates)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<CertificateData>>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(certificates)) => Ok(Some(certificates)),
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
impl<S: Storage + Send + Sync> CertificateRepositoryTrait for CertificateRepository<S> {
    async fn save_certificates(
        &self,
        key: &str,
        certificates: &Vec<CertificateData>,
    ) -> Result<(), RepositoryError> {
        Repository::save(self, key, certificates).await
    }

    async fn get_certificates(
        &self,
        key: &str,
    ) -> Result<Option<Vec<CertificateData>>, RepositoryError> {
        Repository::get(self, key).await
    }

    async fn delete_certificates(&self, key: &str) -> Result<(), RepositoryError> {
        Repository::delete(self, key).await
    }

    async fn add_certificate(
        &self,
        key: &str,
        certificate: CertificateData,
    ) -> Result<(), RepositoryError> {
        let mut certificates = self.get_certificates(key).await?.unwrap_or_default();
        certificates.push(certificate);
        self.save_certificates(key, &certificates).await
    }

    async fn list_certificates(&self, key: &str) -> Result<Vec<CertificateData>, RepositoryError> {
        self.get_certificates(key)
            .await
            .map(|opt| opt.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::memory_storage::MemoryStorage;
    use konnektoren_core::certificates::CertificateData;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_certificate_repository() {
        let storage = MemoryStorage::default();
        let repo = CertificateRepository::new(storage);
        let key = "test_certificates";

        // Test adding a certificate
        let certificate = CertificateData {
            profile_name: "Alice".to_string(),
            ..Default::default()
        };
        repo.add_certificate(key, certificate.clone())
            .await
            .unwrap();

        // Test getting certificates
        let certificates = repo.list_certificates(key).await.unwrap();
        assert_eq!(certificates.len(), 1);
        assert_eq!(certificates[0].profile_name, "Alice");

        // Test saving multiple certificates
        let new_certificates = vec![
            CertificateData {
                profile_name: "Bob".to_string(),
                ..Default::default()
            },
            CertificateData {
                profile_name: "Charlie".to_string(),
                ..Default::default()
            },
        ];
        repo.save_certificates(key, &new_certificates)
            .await
            .unwrap();

        // Test getting updated certificates
        let updated_certificates = repo.list_certificates(key).await.unwrap();
        assert_eq!(updated_certificates.len(), 2);
        assert_eq!(updated_certificates[0].profile_name, "Bob");
        assert_eq!(updated_certificates[1].profile_name, "Charlie");

        // Test deleting certificates
        repo.delete_certificates(key).await.unwrap();
        let empty_certificates = repo.list_certificates(key).await.unwrap();
        assert!(empty_certificates.is_empty());
    }
}
