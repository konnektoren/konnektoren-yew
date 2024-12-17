use crate::i18n::{I18nConfig, SelectedLanguage};
use crate::model::Settings;
use crate::providers::use_settings;
use gloo::utils::window;
use web_sys::UrlSearchParams;
use yew::prelude::*;
use yew_i18n::{YewI18n, YewI18nConfig};

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub i18n: UseStateHandle<YewI18n>,
    pub selected_language: SelectedLanguage,
}

#[derive(Properties, Clone, PartialEq)]
pub struct I18nProviderProps {
    pub config: I18nConfig,
    pub children: Children,
}

fn get_url_language(supported_languages: &[&str]) -> Option<String> {
    let window = window();
    let search = window.location().search().unwrap();
    let search_params = UrlSearchParams::new_with_str(&search).ok()?;

    let lang = search_params.get("lang")?;
    if supported_languages.contains(&lang.as_str()) {
        Some(lang)
    } else {
        None
    }
}

fn get_browser_language(supported_languages: &[&str]) -> Option<String> {
    let language = window().navigator().language().unwrap();
    if supported_languages.contains(&language.as_str()) {
        Some(language)
    } else {
        None
    }
}

fn determine_language(config: &I18nConfig, settings: &UseStateHandle<Settings>) -> String {
    // Priority: URL query param → config → browser language
    get_url_language(&config.supported_languages)
        .or_else(|| {
            if config
                .supported_languages
                .contains(&settings.language.as_str())
            {
                Some(settings.language.clone())
            } else {
                None
            }
        })
        .or_else(|| get_browser_language(&config.supported_languages))
        .unwrap_or_else(|| config.default_language.clone())
}

fn create_i18n(config: &I18nConfig, language: &str) -> YewI18n {
    let mut i18n = YewI18n::new(
        YewI18nConfig {
            supported_languages: config.supported_languages.clone(),
            translations: config.translations.clone(),
        },
        config.translations.clone(),
    )
    .expect("Failed to initialize YewI18n");

    i18n.set_translation_language(language)
        .expect("Failed to set translation language");

    i18n
}

#[function_component(I18nProvider)]
pub fn i18n_provider(props: &I18nProviderProps) -> Html {
    let settings = use_settings();

    let selected_language = {
        let language = determine_language(&props.config, &settings);
        SelectedLanguage::new(&language)
    };

    let i18n_ctx = use_state(|| create_i18n(&props.config, &selected_language.language));

    {
        let i18n_ctx = i18n_ctx.clone();
        let settings = settings.clone();
        let config = props.config.clone();

        use_effect_with(settings.clone(), move |settings| {
            let language = determine_language(&config, settings);
            let mut i18n = (*i18n_ctx).clone();
            i18n.set_translation_language(&language)
                .expect("Failed to set translation language");
            i18n_ctx.set(i18n);
        });
    }

    let context = I18nContext {
        i18n: i18n_ctx,
        selected_language,
    };

    html! {
        <ContextProvider<I18nContext> {context}>
            { for props.children.iter() }
        </ContextProvider<I18nContext>>
    }
}

#[hook]
pub fn use_i18n() -> UseStateHandle<YewI18n> {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .i18n
        .clone()
}

#[hook]
pub fn use_selected_language() -> SelectedLanguage {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .selected_language
}
