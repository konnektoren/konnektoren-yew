use super::RepositoryConfig;
use crate::repository::{
    CertificateRepositoryTrait, InboxRepositoryTrait, ProfileRepositoryTrait,
    SessionRepositoryTrait, SettingsRepositoryTrait,
};
use konnektoren_core::certificates::CertificateData;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct RepositoryContext {
    pub certificate_repository: Arc<dyn CertificateRepositoryTrait>,
    pub settings_repository: Arc<dyn SettingsRepositoryTrait>,
    pub profile_repository: Arc<dyn ProfileRepositoryTrait>,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub certificates: Arc<RwLock<Vec<CertificateData>>>,
}

impl PartialEq for RepositoryContext {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.certificate_repository, &other.certificate_repository)
            && Arc::ptr_eq(&self.settings_repository, &other.settings_repository)
            && Arc::ptr_eq(&self.profile_repository, &other.profile_repository)
            && Arc::ptr_eq(&self.inbox_repository, &other.inbox_repository)
            && Arc::ptr_eq(&self.session_repository, &other.session_repository)
            && Arc::ptr_eq(&self.certificates, &other.certificates)
    }
}

impl RepositoryContext {
    pub fn new(config: RepositoryConfig) -> Self {
        Self {
            certificate_repository: config.certificate_repository,
            settings_repository: config.settings_repository,
            profile_repository: config.profile_repository,
            inbox_repository: config.inbox_repository,
            session_repository: config.session_repository,
            certificates: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
