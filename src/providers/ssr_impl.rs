use crate::model::{Inbox, Settings};
use crate::providers::{
    CertificatesContext, InboxContext, ProfileContext, RepositoryContext, SessionContext,
    SettingsContext,
};
use crate::repository::{
    CertificateRepositoryTrait, InboxRepositoryTrait, ProfileRepositoryTrait,
    SessionRepositoryTrait, SettingsRepositoryTrait,
};
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::prelude::{PlayerProfile, Session};
use std::sync::Arc;
use yew::prelude::*;

#[cfg(feature = "ssr")]
pub mod ssr_impl {
    use super::*;
    use crate::repository::{
        RepositoryError, CERTIFICATE_STORAGE_KEY, INBOX_STORAGE_KEY, PROFILE_STORAGE_KEY,
        SESSION_STORAGE_KEY, SETTINGS_STORAGE_KEY,
    };
    use async_trait::async_trait;

    // Dummy struct that implements all repository traits
    pub struct DummyRepository;

    #[async_trait]
    impl CertificateRepositoryTrait for DummyRepository {
        async fn save_certificates(
            &self,
            _: &str,
            _: &Vec<CertificateData>,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn get_certificates(
            &self,
            _: &str,
        ) -> Result<Option<Vec<CertificateData>>, RepositoryError> {
            Ok(Some(vec![]))
        }
        async fn delete_certificates(&self, _: &str) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn add_certificate(
            &self,
            _: &str,
            _: CertificateData,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn list_certificates(
            &self,
            _: &str,
        ) -> Result<Vec<CertificateData>, RepositoryError> {
            Ok(vec![])
        }
    }

    #[async_trait]
    impl SettingsRepositoryTrait for DummyRepository {
        async fn save_settings(&self, _: &str, _: &Settings) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn get_settings(&self, _: &str) -> Result<Option<Settings>, RepositoryError> {
            Ok(Some(Settings::default()))
        }
        async fn delete_settings(&self, _: &str) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    #[async_trait]
    impl ProfileRepositoryTrait for DummyRepository {
        async fn save_profile(&self, _: &str, _: &PlayerProfile) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn get_profile(&self, _: &str) -> Result<Option<PlayerProfile>, RepositoryError> {
            Ok(Some(PlayerProfile::default()))
        }
        async fn delete_profile(&self, _: &str) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn update_profile(&self, _: &str, _: &PlayerProfile) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    #[async_trait]
    impl InboxRepositoryTrait for DummyRepository {
        async fn save_inbox(&self, _: &str, _: &Inbox) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn get_inbox(&self, _: &str) -> Result<Option<Inbox>, RepositoryError> {
            Ok(Some(Inbox::default()))
        }
        async fn delete_inbox(&self, _: &str) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn merge_inbox(&self, _: &str, _: &Inbox) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn add_message(
            &self,
            _: &str,
            _: yew_chat::prelude::Message,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn mark_as_read(&self, _: &str, _: &str) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    #[async_trait]
    impl SessionRepositoryTrait for DummyRepository {
        async fn save_session(&self, _: &str, _: &Session) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn get_session(&self, _: &str) -> Result<Option<Session>, RepositoryError> {
            Ok(Some(Session::default()))
        }
        async fn delete_session(&self, _: &str) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn update_session(&self, _: &str, _: &Session) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    pub fn certificate_repository() -> Arc<dyn CertificateRepositoryTrait> {
        Arc::new(DummyRepository) as Arc<dyn CertificateRepositoryTrait>
    }

    pub fn settings_repository() -> Arc<dyn SettingsRepositoryTrait> {
        Arc::new(DummyRepository) as Arc<dyn SettingsRepositoryTrait>
    }

    pub fn profile_repository() -> Arc<dyn ProfileRepositoryTrait> {
        Arc::new(DummyRepository) as Arc<dyn ProfileRepositoryTrait>
    }

    pub fn inbox_repository() -> Arc<dyn InboxRepositoryTrait> {
        Arc::new(DummyRepository) as Arc<dyn InboxRepositoryTrait>
    }

    pub fn session_repository() -> Arc<dyn SessionRepositoryTrait> {
        Arc::new(DummyRepository) as Arc<dyn SessionRepositoryTrait>
    }
}
