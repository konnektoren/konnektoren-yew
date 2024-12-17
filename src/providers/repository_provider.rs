use super::{
    CertificatesProvider, InboxProvider, ProfileProvider, RepositoryContext, SessionProvider,
    SettingsProvider,
};
use crate::model::SessionInitializer;
use crate::repository::{
    CertificateRepository, CertificateRepositoryTrait, InboxRepository, InboxRepositoryTrait,
    ProfileRepository, ProfileRepositoryTrait, SessionRepository, SessionRepositoryTrait,
    SettingsRepository, SettingsRepositoryTrait, Storage,
};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone)]
pub struct RepositoryConfig {
    pub certificate_repository: Arc<dyn CertificateRepositoryTrait>,
    pub settings_repository: Arc<dyn SettingsRepositoryTrait>,
    pub profile_repository: Arc<dyn ProfileRepositoryTrait>,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session_initializer: Arc<dyn SessionInitializer>,
}

impl PartialEq for RepositoryConfig {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.certificate_repository, &other.certificate_repository)
            && Arc::ptr_eq(&self.settings_repository, &other.settings_repository)
            && Arc::ptr_eq(&self.profile_repository, &other.profile_repository)
            && Arc::ptr_eq(&self.inbox_repository, &other.inbox_repository)
            && Arc::ptr_eq(&self.session_repository, &other.session_repository)
            && Arc::ptr_eq(&self.session_initializer, &other.session_initializer)
    }
}

pub fn create_repositories<S: Storage + Send + Sync + 'static>(
    storage: S,
    session_initializer: Arc<dyn SessionInitializer>,
) -> RepositoryConfig {
    RepositoryConfig {
        certificate_repository: Arc::new(CertificateRepository::new(storage.clone()))
            as Arc<dyn CertificateRepositoryTrait>,
        settings_repository: Arc::new(SettingsRepository::new(storage.clone()))
            as Arc<dyn SettingsRepositoryTrait>,
        profile_repository: Arc::new(ProfileRepository::new(storage.clone()))
            as Arc<dyn ProfileRepositoryTrait>,
        inbox_repository: Arc::new(InboxRepository::new(storage.clone()))
            as Arc<dyn InboxRepositoryTrait>,
        session_repository: Arc::new(SessionRepository::new(storage))
            as Arc<dyn SessionRepositoryTrait>,
        session_initializer,
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RepositoryProviderProps {
    pub children: Children,
    pub config: RepositoryConfig,
}

#[function_component(RepositoryProvider)]
pub fn repository_provider(props: &RepositoryProviderProps) -> Html {
    let context = RepositoryContext::new(props.config.clone());

    html! {
        <ContextProvider<RepositoryContext> context={context.clone()}>
            <SessionProvider session_repository={context.session_repository.clone()}
                session_initializer={props.config.session_initializer.clone()}>
                <ProfileProvider profile_repository={context.profile_repository}>
                    <SettingsProvider settings_repository={context.settings_repository}>
                        <CertificatesProvider certificates_repository={context.certificate_repository}>
                        <InboxProvider inbox_repository={context.inbox_repository}>
                            { for props.children.iter() }
                        </InboxProvider>
                        </CertificatesProvider>
                    </SettingsProvider>
                </ProfileProvider>
            </SessionProvider>
        </ContextProvider<RepositoryContext>>
    }
}
