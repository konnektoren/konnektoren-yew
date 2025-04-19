use crate::model::Settings;
use crate::repository::{SETTINGS_STORAGE_KEY, SettingsRepositoryTrait};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SettingsContext {
    pub settings: UseStateHandle<Settings>,
}

#[derive(Clone, Properties)]
pub struct SettingsProviderProps {
    pub children: Children,
    pub settings_repository: Arc<dyn SettingsRepositoryTrait>,
}

impl PartialEq for SettingsProviderProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.settings_repository, &other.settings_repository)
    }
}

#[function_component(SettingsProvider)]
pub fn settings_provider(props: &SettingsProviderProps) -> Html {
    let settings = use_state(Settings::default);

    {
        let settings = settings.clone();
        let settings_repository = props.settings_repository.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_settings)) =
                    settings_repository.get_settings(SETTINGS_STORAGE_KEY).await
                {
                    settings.set(loaded_settings);
                }
            });
            || ()
        });
    }

    {
        let settings_repository = props.settings_repository.clone();
        let settings = settings.clone();

        use_effect_with(settings.clone(), move |_| {
            let settings = settings.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let settings = settings.clone();
                if let Err(e) = settings_repository
                    .save_settings(SETTINGS_STORAGE_KEY, &settings)
                    .await
                {
                    log::error!("Failed to save settings: {:?}", e);
                }
            });
            || ()
        });
    }

    let context = SettingsContext { settings };

    html! {
        <ContextProvider<SettingsContext> {context}>
            {props.children.clone()}
        </ContextProvider<SettingsContext>>
    }
}
