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
use super::ssr_impl::ssr_impl;

#[hook]
pub fn use_certificate_repository() -> Arc<dyn CertificateRepositoryTrait> {
    #[cfg(feature = "ssr")]
    {
        static INSTANCE: std::sync::OnceLock<Arc<dyn CertificateRepositoryTrait>> =
            std::sync::OnceLock::new();
        INSTANCE
            .get_or_init(|| ssr_impl::certificate_repository())
            .clone()
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<RepositoryContext>()
            .expect("RepositoryContext not found")
            .certificate_repository
            .clone()
    }
}

#[hook]
pub fn use_settings_repository() -> Arc<dyn SettingsRepositoryTrait> {
    #[cfg(feature = "ssr")]
    {
        static INSTANCE: std::sync::OnceLock<Arc<dyn SettingsRepositoryTrait>> =
            std::sync::OnceLock::new();
        INSTANCE
            .get_or_init(|| ssr_impl::settings_repository())
            .clone()
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<RepositoryContext>()
            .expect("RepositoryContext not found")
            .settings_repository
    }
}

#[hook]
pub fn use_profile_repository() -> Arc<dyn ProfileRepositoryTrait> {
    #[cfg(feature = "ssr")]
    {
        static INSTANCE: std::sync::OnceLock<Arc<dyn ProfileRepositoryTrait>> =
            std::sync::OnceLock::new();
        INSTANCE
            .get_or_init(|| ssr_impl::profile_repository())
            .clone()
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<RepositoryContext>()
            .expect("RepositoryContext not found")
            .profile_repository
    }
}

#[hook]
pub fn use_inbox_repository() -> Arc<dyn InboxRepositoryTrait> {
    #[cfg(feature = "ssr")]
    {
        static INSTANCE: std::sync::OnceLock<Arc<dyn InboxRepositoryTrait>> =
            std::sync::OnceLock::new();
        INSTANCE
            .get_or_init(|| ssr_impl::inbox_repository())
            .clone()
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<RepositoryContext>()
            .expect("RepositoryContext not found")
            .inbox_repository
    }
}

#[hook]
pub fn use_session_repository() -> Arc<dyn SessionRepositoryTrait> {
    #[cfg(feature = "ssr")]
    {
        static INSTANCE: std::sync::OnceLock<Arc<dyn SessionRepositoryTrait>> =
            std::sync::OnceLock::new();
        INSTANCE
            .get_or_init(|| ssr_impl::session_repository())
            .clone()
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<RepositoryContext>()
            .expect("RepositoryContext not found")
            .session_repository
    }
}

#[hook]
pub fn use_session() -> UseStateHandle<Session> {
    use_context::<SessionContext>()
        .expect("SessionContext not found")
        .session
}

#[hook]
pub fn use_profile() -> UseStateHandle<PlayerProfile> {
    #[cfg(feature = "ssr")]
    {
        use_state(PlayerProfile::default)
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<ProfileContext>()
            .expect("ProfileContext not found")
            .profile
    }
}

#[hook]
pub fn use_inbox() -> UseStateHandle<Inbox> {
    #[cfg(feature = "ssr")]
    {
        use_state(Inbox::default)
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<InboxContext>()
            .expect("InboxContext not found")
            .inbox
    }
}

#[hook]
pub fn use_settings() -> UseStateHandle<Settings> {
    #[cfg(feature = "ssr")]
    {
        use_state(Settings::default)
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<SettingsContext>()
            .expect("SettingsContext not found")
            .settings
    }
}

#[hook]
pub fn use_certificates() -> UseStateHandle<Vec<CertificateData>> {
    #[cfg(feature = "ssr")]
    {
        use_state(Vec::new)
    }
    #[cfg(not(feature = "ssr"))]
    {
        use_context::<CertificatesContext>()
            .expect("CertificatesContext not found")
            .certificates
    }
}
