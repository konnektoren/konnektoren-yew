use super::SelectedLanguage;
use crate::model::Settings;
use crate::providers::use_settings;
use gloo::utils::window;
use konnektoren_platform::i18n::I18nConfig;
use konnektoren_platform::prelude::Language;
use web_sys::UrlSearchParams;
use yew::prelude::*;

fn get_url_language(supported_languages: &[Language]) -> Option<Language> {
    let window = window();
    let search = window.location().search().unwrap();
    let search_params = UrlSearchParams::new_with_str(&search).ok()?;

    let lang = search_params.get("lang")?;
    if supported_languages.iter().any(|l| l.code() == lang) {
        Some(Language::from_code(&lang))
    } else {
        None
    }
}

fn get_path_language(supported_languages: &[Language]) -> Option<Language> {
    let window = window();
    let path = window.location().pathname().unwrap_or_default();
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();

    parts.first().and_then(|first_part| {
        if supported_languages.iter().any(|l| l.code() == *first_part) {
            Some(Language::from_code(first_part))
        } else {
            None
        }
    })
}

fn get_browser_language(supported_languages: &[Language]) -> Option<Language> {
    let language = window().navigator().language().unwrap();
    if supported_languages.iter().any(|l| l.code() == language) {
        Some(Language::from_code(&language))
    } else {
        None
    }
}

fn determine_language(config: &I18nConfig, settings: &UseStateHandle<Settings>) -> Language {
    let supported_languages = config.supported_languages();

    // Priority: URL path → URL query param → settings → browser language → default
    get_path_language(&supported_languages)
        .or_else(|| get_url_language(&supported_languages))
        .or_else(|| {
            let settings_lang = Language::from_code(&settings.language);
            if supported_languages.contains(&settings_lang) {
                Some(settings_lang)
            } else {
                None
            }
        })
        .or_else(|| get_browser_language(&supported_languages))
        .unwrap_or_else(|| config.default_language.clone())
}

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub config: UseStateHandle<I18nConfig>,
    pub selected_language: SelectedLanguage,
}

#[derive(Properties, Clone, PartialEq)]
pub struct I18nProviderProps {
    pub config: I18nConfig,
    pub children: Children,
}

#[function_component(I18nProvider)]
pub fn i18n_provider(props: &I18nProviderProps) -> Html {
    let settings = use_settings();

    // Initialize selected_language first
    let selected_language = {
        let language = determine_language(&props.config, &settings);
        SelectedLanguage::new(&language.code())
    };

    // Initialize config state with props
    let config_ctx = use_state(|| props.config.clone());

    {
        let config_ctx = config_ctx.clone();
        let settings = settings.clone();
        let props_config = props.config.clone();

        use_effect_with(settings.clone(), move |settings| {
            let language = determine_language(&props_config, settings);
            let mut config = props_config.clone();
            // Update default language if needed
            if config.default_language.code() != language.code() {
                config.default_language = language;
            }
            config_ctx.set(config);
        });
    }

    let context = I18nContext {
        config: config_ctx,
        selected_language,
    };

    html! {
        <ContextProvider<I18nContext> {context}>
            { for props.children.iter() }
        </ContextProvider<I18nContext>>
    }
}
#[hook]
pub fn use_i18n() -> UseStateHandle<I18nConfig> {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .config
}

#[hook]
pub fn use_selected_language() -> SelectedLanguage {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .selected_language
}

#[hook]
pub fn use_translation() -> impl Fn(&str) -> String {
    let config = use_i18n();
    let selected_language = use_selected_language();

    move |key: &str| config.t_with_lang(key, &selected_language.get())
}
