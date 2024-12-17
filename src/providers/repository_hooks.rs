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

#[hook]
pub fn use_certificate_repository() -> Arc<dyn CertificateRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .certificate_repository
        .clone()
}

#[hook]
pub fn use_settings_repository() -> Arc<dyn SettingsRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .settings_repository
}

#[hook]
pub fn use_profile_repository() -> Arc<dyn ProfileRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .profile_repository
}

#[hook]
pub fn use_inbox_repository() -> Arc<dyn InboxRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .inbox_repository
}

#[hook]
pub fn use_session_repository() -> Arc<dyn SessionRepositoryTrait> {
    use_context::<RepositoryContext>()
        .expect("RepositoryContext not found")
        .session_repository
}

#[hook]
pub fn use_session() -> UseStateHandle<Session> {
    use_context::<SessionContext>()
        .expect("RepositoryContext not found")
        .session
}

#[hook]
pub fn use_profile() -> UseStateHandle<PlayerProfile> {
    use_context::<ProfileContext>()
        .expect("ProfileContext not found")
        .profile
}

#[hook]
pub fn use_inbox() -> UseStateHandle<Inbox> {
    use_context::<InboxContext>()
        .expect("InboxContext not found")
        .inbox
}

#[hook]
pub fn use_settings() -> UseStateHandle<Settings> {
    use_context::<SettingsContext>()
        .expect("SettingsContext not found")
        .settings
}

#[hook]
pub fn use_certificates() -> UseStateHandle<Vec<CertificateData>> {
    use_context::<CertificatesContext>()
        .expect("CertificatesContext not found")
        .certificates
}
