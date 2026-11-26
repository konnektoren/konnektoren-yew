use crate::model::Settings;
use crate::repository::{SETTINGS_STORAGE_KEY, SettingsRepositoryTrait};
use konnektoren_yew_core::{SpeechConfig, SpeechProvider};
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
    // IMPORTANT: use_state must be called OUTSIDE cfg blocks to maintain hook ordering
    let settings = use_state(Settings::default);

    // Load settings (CSR only)
    #[cfg(feature = "csr")]
    {
        let settings = settings.clone();
        let settings_repository = props.settings_repository.clone();

        use_effect_with((), move |_| {
            use wasm_bindgen_futures::spawn_local;

            spawn_local(async move {
                if let Ok(Some(loaded_settings)) =
                    settings_repository.get_settings(SETTINGS_STORAGE_KEY).await
                {
                    settings.set(loaded_settings);
                }
            });
            || ()
        });
    }

    // Save settings (CSR only)
    #[cfg(feature = "csr")]
    {
        let settings_repository = props.settings_repository.clone();
        let settings = settings.clone();

        use_effect_with(settings.clone(), move |_| {
            use wasm_bindgen_futures::spawn_local;

            let settings = settings.clone();
            spawn_local(async move {
                let settings = settings.clone();
                if let Err(e) = settings_repository
                    .save_settings(SETTINGS_STORAGE_KEY, &settings)
                    .await
                {
                    tracing::error!("Failed to save settings: {:?}", e);
                }
            });
            || ()
        });
    }

    let speech_override = use_context::<SpeechConfig>();
    let speech = SpeechConfig {
        volume: settings.sound_volume,
        ..SpeechConfig::default()
    };
    let context = SettingsContext { settings };

    html! {
        <ContextProvider<SettingsContext> {context}>
            if speech_override.is_some() {
                {props.children.clone()}
            } else {
                <SpeechProvider config={speech}>{props.children.clone()}</SpeechProvider>
            }
        </ContextProvider<SettingsContext>>
    }
}
