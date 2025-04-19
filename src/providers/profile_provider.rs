use crate::repository::{PROFILE_STORAGE_KEY, ProfileRepositoryTrait};
use konnektoren_core::prelude::PlayerProfile;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ProfileContext {
    pub profile: UseStateHandle<PlayerProfile>,
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

    // Load profile
    {
        let profile = profile.clone();
        let profile_repository = props.profile_repository.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_profile)) =
                    profile_repository.get_profile(PROFILE_STORAGE_KEY).await
                {
                    profile.set(loaded_profile);
                }
            });
            || ()
        });
    }

    {
        let profile_repository = props.profile_repository.clone();
        let profile = profile.clone();

        use_effect_with(profile.clone(), move |_| {
            let profile = profile.clone();
            wasm_bindgen_futures::spawn_local(async move {
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
