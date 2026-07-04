#[cfg(feature = "csr")]
use super::supported_language_from_candidates;
use super::{SelectedLanguage, supported_language_code};
use crate::model::Settings;
use crate::providers::use_settings;
use konnektoren_rs::platform::i18n::I18nConfig;
use konnektoren_rs::platform::prelude::Language;
use yew::prelude::*;

fn language_from_code(code: &str, supported_languages: &[Language]) -> Option<Language> {
    let supported_codes: Vec<&str> = supported_languages
        .iter()
        .map(|language| language.code())
        .collect();
    let code = supported_language_code(Some(code), &supported_codes)?;
    supported_languages
        .iter()
        .find(|language| language.code() == code)
        .cloned()
}

#[cfg(feature = "csr")]
fn language_from_candidates<'a>(
    candidates: impl IntoIterator<Item = &'a str>,
    supported_languages: &[Language],
) -> Option<Language> {
    let supported_codes: Vec<&str> = supported_languages
        .iter()
        .map(|language| language.code())
        .collect();
    let code = supported_language_from_candidates(candidates, &supported_codes)?;
    supported_languages
        .iter()
        .find(|language| language.code() == code)
        .cloned()
}

#[cfg(feature = "csr")]
fn get_url_language(supported_languages: &[Language]) -> Option<Language> {
    use gloo::utils::window;
    use web_sys::UrlSearchParams;

    let window = window();
    let search = window.location().search().ok()?;
    let search_params = UrlSearchParams::new_with_str(&search).ok()?;

    let lang = search_params.get("lang")?;
    language_from_code(&lang, supported_languages)
}

#[cfg(not(feature = "csr"))]
fn get_url_language(_supported_languages: &[Language]) -> Option<Language> {
    None
}

#[cfg(feature = "csr")]
fn get_path_language(supported_languages: &[Language]) -> Option<Language> {
    use gloo::utils::window;

    let window = window();
    let path = window.location().pathname().unwrap_or_default();
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();

    parts
        .first()
        .and_then(|first_part| language_from_code(first_part, supported_languages))
}

#[cfg(all(not(feature = "csr"), feature = "ssr"))]
fn get_path_language(supported_languages: &[Language]) -> Option<Language> {
    // In SSG/SSR, the current path is provided via the YEW_SSG_CURRENT_PATH env var.
    // Extract the first path segment and check if it is a supported language code
    // (e.g. "/es/404/" → "es").
    let path = std::env::var("YEW_SSG_CURRENT_PATH").ok()?;
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
    parts
        .first()
        .and_then(|first_part| language_from_code(first_part, supported_languages))
}

#[cfg(all(not(feature = "csr"), not(feature = "ssr")))]
fn get_path_language(_supported_languages: &[Language]) -> Option<Language> {
    None
}

#[cfg(feature = "csr")]
fn get_browser_language(supported_languages: &[Language]) -> Option<Language> {
    use gloo::utils::window;
    use wasm_bindgen::JsValue;

    let navigator = window().navigator();
    let mut candidates = Vec::new();
    if let Ok(languages) = js_sys::Reflect::get(navigator.as_ref(), &JsValue::from_str("languages"))
    {
        let languages = js_sys::Array::from(&languages);
        candidates.extend(languages.iter().filter_map(|lang| lang.as_string()));
    }
    if let Some(language) = navigator.language() {
        candidates.push(language);
    }

    language_from_candidates(candidates.iter().map(String::as_str), supported_languages)
}

#[cfg(not(feature = "csr"))]
fn get_browser_language(_supported_languages: &[Language]) -> Option<Language> {
    None
}

#[cfg(feature = "ssr")]
fn get_env_language(supported_languages: &[Language]) -> Option<Language> {
    std::env::var("LANG").ok().and_then(|lang| {
        let language = language_from_code(&lang, supported_languages)?;
        tracing::debug!(
            "🌐 Using language from environment: LANG={} resolved to {}",
            lang,
            language.code()
        );
        Some(language)
    })
}

fn determine_language(config: &I18nConfig, settings: &UseStateHandle<Settings>) -> Language {
    let supported_languages = config.supported_languages();

    // Path language takes highest priority (explicit /es/… beats any env/settings fallback)
    if let Some(path_lang) = get_path_language(&supported_languages) {
        return path_lang;
    }

    // Priority in SSR: LANG environment variable
    #[cfg(feature = "ssr")]
    {
        if let Some(env_lang) = get_env_language(&supported_languages) {
            return env_lang;
        }
    }

    // Priority: URL query param → settings → browser language → default
    get_url_language(&supported_languages)
        .or_else(|| {
            if settings.language.is_empty() {
                return None;
            }
            language_from_code(&settings.language, &supported_languages)
        })
        .or_else(|| get_browser_language(&supported_languages))
        .unwrap_or_else(|| config.default_language.clone())
}

#[cfg(not(feature = "csr"))]
fn determine_browser_language(config: &I18nConfig, default_lang_code: Option<&str>) -> Language {
    let supported_languages = config.supported_languages();

    // Priority: URL path language (highest — a path like /es/… is definitively Spanish)
    // This must come before the LANG env var check so that localized SSG routes such as
    // /es/404/ are rendered in Spanish even though LANG may be set to "en" as a fallback.
    if let Some(path_lang) = get_path_language(&supported_languages) {
        return path_lang;
    }

    // Priority in SSR: LANG environment variable (fallback when the path carries no lang prefix)
    #[cfg(feature = "ssr")]
    {
        if let Some(env_lang) = get_env_language(&supported_languages) {
            return env_lang;
        }
    }

    // Priority: URL query param → browser language → caller default → config default
    get_url_language(&supported_languages)
        .or_else(|| get_browser_language(&supported_languages))
        .or_else(|| {
            default_lang_code.and_then(|code| language_from_code(code, &supported_languages))
        })
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
    #[cfg(feature = "ssr")]
    {
        use crate::i18n::log_language_info;
        log_language_info("I18nProvider initialization");
    }

    let settings = use_settings();

    let config_ctx = use_state(|| {
        let language = determine_language(&props.config, &settings);
        let mut config = props.config.clone();
        config.default_language = language;
        config
    });

    {
        let config_ctx = config_ctx.clone();
        let settings = settings.clone();
        let props_config = props.config.clone();

        use_effect_with(settings.clone(), move |settings| {
            let language = determine_language(&props_config, settings);
            let mut config = props_config.clone();
            if config.default_language.code() != language.code() {
                config.default_language = language;
                config_ctx.set(config);
            }
            || ()
        });
    }

    // Derived from config_ctx so it stays in sync when config updates
    let selected_language = SelectedLanguage::new(config_ctx.default_language.code());

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

#[function_component(BrowserI18nProvider)]
pub fn browser_i18n_provider(props: &I18nProviderProps) -> Html {
    use crate::providers::SettingsContext;

    let initial_config = props.config.clone();
    let supported_languages = initial_config.supported_languages();

    // Optionally read settings — returns None when SettingsProvider is not in the tree
    let settings_ctx = use_context::<SettingsContext>();
    let settings_language = settings_ctx
        .as_ref()
        .map(|ctx| ctx.settings.language.clone())
        .unwrap_or_default();

    // Priority: URL path → URL query → settings → browser → config default
    let initial_language = {
        #[cfg(feature = "csr")]
        {
            get_path_language(&supported_languages)
                .or_else(|| get_url_language(&supported_languages))
                .or_else(|| {
                    if settings_language.is_empty() {
                        return None;
                    }
                    language_from_code(&settings_language, &supported_languages)
                })
                .or_else(|| get_browser_language(&supported_languages))
                .unwrap_or_else(|| initial_config.default_language.clone())
        }
        #[cfg(not(feature = "csr"))]
        {
            determine_browser_language(&initial_config, None)
        }
    };

    let config_ctx = use_state(|| {
        let mut config = initial_config.clone();
        config.default_language = initial_language;
        config
    });

    // React to settings language changes (fires when user picks a language in SelectLanguage)
    {
        let config_ctx = config_ctx.clone();
        let supported_languages = supported_languages.clone();

        use_effect_with(settings_language, move |lang| {
            let url_overrides = {
                #[cfg(feature = "csr")]
                {
                    get_path_language(&supported_languages).is_some()
                        || get_url_language(&supported_languages).is_some()
                }
                #[cfg(not(feature = "csr"))]
                {
                    false
                }
            };

            if !lang.is_empty()
                && !url_overrides
                && let Some(lang_obj) = language_from_code(lang, &supported_languages)
                && config_ctx.default_language.code() != lang_obj.code()
            {
                let mut config = (*config_ctx).clone();
                config.default_language = lang_obj;
                config_ctx.set(config);
            }
            || ()
        });
    }

    // Listen for URL navigation changes (popstate)
    {
        let config_ctx = config_ctx.clone();
        let initial_config = initial_config.clone();

        use_effect_with((), move |_| {
            #[cfg(not(feature = "csr"))]
            {
                let _ = &config_ctx;
                let _ = &initial_config;
            }
            #[cfg(feature = "csr")]
            {
                use gloo::events::EventListener;
                use gloo::utils::window;

                let window = window();
                let listener = EventListener::new(&window, "popstate", move |_| {
                    let supported = initial_config.supported_languages();
                    let lang =
                        get_path_language(&supported).or_else(|| get_url_language(&supported));
                    if let Some(lang) = lang {
                        if config_ctx.default_language.code() != lang.code() {
                            let mut config = (*config_ctx).clone();
                            config.default_language = lang;
                            config_ctx.set(config);
                        }
                    }
                });
                return move || drop(listener);
            }
            #[cfg(not(feature = "csr"))]
            {
                || ()
            }
        });
    }

    // Derived from config_ctx so it stays in sync when config updates
    let selected_language = SelectedLanguage::new(config_ctx.default_language.code());

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
    // Always use the context version which has the full translations loaded
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .selected_language
}
