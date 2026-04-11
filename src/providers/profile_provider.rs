use crate::repository::{PROFILE_STORAGE_KEY, ProfileRepositoryTrait};
use konnektoren_core::prelude::PlayerProfile;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ProfileContext {
    pub profile: UseStateHandle<PlayerProfile>,
}

#[cfg(feature = "csr")]
fn should_persist_profile(is_hydrated: bool) -> bool {
    is_hydrated
}

#[derive(Properties)]
pub struct ProfileProviderProps {
    pub children: Children,
    pub profile_repository: Arc<dyn ProfileRepositoryTrait>,
}

impl PartialEq for ProfileProviderProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.profile_repository, &other.profile_repository)
    }
}

#[function_component(ProfileProvider)]
pub fn profile_provider(props: &ProfileProviderProps) -> Html {
    let profile = use_state(PlayerProfile::default);
    #[cfg(feature = "csr")]
    let is_hydrated = use_state(|| false);

    // Load profile (CSR only)
    #[cfg(feature = "csr")]
    {
        let profile = profile.clone();
        let profile_repository = props.profile_repository.clone();
        let is_hydrated = is_hydrated.clone();

        use_effect_with((), move |_| {
            use wasm_bindgen_futures::spawn_local;

            spawn_local(async move {
                if let Ok(Some(loaded_profile)) =
                    profile_repository.get_profile(PROFILE_STORAGE_KEY).await
                {
                    profile.set(loaded_profile);
                }
                is_hydrated.set(true);
            });
            || ()
        });
    }

    // Save profile (CSR only)
    #[cfg(feature = "csr")]
    {
        let profile_repository = props.profile_repository.clone();
        let profile = profile.clone();
        let is_hydrated = is_hydrated.clone();

        use_effect_with((profile.clone(), *is_hydrated), move |_| {
            use wasm_bindgen_futures::spawn_local;

            let profile = profile.clone();
            let is_hydrated = *is_hydrated;
            spawn_local(async move {
                if !should_persist_profile(is_hydrated) {
                    return;
                }

                let profile = profile.clone();
                if let Err(e) = profile_repository
                    .update_profile(PROFILE_STORAGE_KEY, &profile)
                    .await
                {
                    log::error!("Failed to save profile: {:?}", e);
                }
            });
            || ()
        });
    }

    let context = ProfileContext { profile };

    html! {
        <ContextProvider<ProfileContext> {context}>
            { for props.children.iter() }
        </ContextProvider<ProfileContext>>
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "csr")]
    use super::should_persist_profile;

    #[cfg(feature = "csr")]
    #[test]
    fn profile_is_not_persisted_before_hydration() {
        assert!(!should_persist_profile(false));
        assert!(should_persist_profile(true));
    }
}
