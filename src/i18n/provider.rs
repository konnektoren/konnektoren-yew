use super::SelectedLanguage;
use crate::model::Settings;
use crate::providers::use_settings;
use konnektoren_platform::i18n::I18nConfig;
use konnektoren_platform::prelude::Language;
use yew::prelude::*;

#[cfg(feature = "csr")]
fn get_url_language(supported_languages: &[Language]) -> Option<Language> {
    use gloo::utils::window;
    use web_sys::UrlSearchParams;

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

    parts.first().and_then(|first_part| {
        if supported_languages.iter().any(|l| l.code() == *first_part) {
            Some(Language::from_code(first_part))
        } else {
            None
        }
    })
}

#[cfg(not(feature = "csr"))]
fn get_path_language(_supported_languages: &[Language]) -> Option<Language> {
    None
}

#[cfg(feature = "csr")]
fn get_browser_language(supported_languages: &[Language]) -> Option<Language> {
    use gloo::utils::window;

    let language = window().navigator().language().unwrap_or_default();
    if supported_languages.iter().any(|l| l.code() == language) {
        Some(Language::from_code(&language))
    } else {
        None
    }
}

#[cfg(not(feature = "csr"))]
fn get_browser_language(_supported_languages: &[Language]) -> Option<Language> {
    None
}

#[cfg(feature = "ssr")]
fn get_env_language() -> Option<Language> {
    std::env::var("LANG").ok().map(|lang| {
        log::debug!("🌐 Using language from environment: LANG={}", lang);
        Language::from_code(&lang)
    })
}

fn determine_language(config: &I18nConfig, settings: &UseStateHandle<Settings>) -> Language {
    let supported_languages = config.supported_languages();

    // Priority in SSR: environment variable
    #[cfg(feature = "ssr")]
    {
        if let Some(env_lang) = get_env_language() {
            return env_lang;
        }
    }

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

fn determine_browser_language(config: &I18nConfig, default_lang_code: Option<&str>) -> Language {
    let supported_languages = config.supported_languages();

    // If a default language code was provided, try to use it
    if let Some(code) = default_lang_code {
        let lang = Language::from_code(code);
        if supported_languages.contains(&lang) {
            return lang;
        }
    }

    // Priority in SSR: environment variable
    #[cfg(feature = "ssr")]
    {
        if let Some(env_lang) = get_env_language() {
            return env_lang;
        }
    }

    // Priority: URL path → URL query param → browser language → default
    get_path_language(&supported_languages)
        .or_else(|| get_url_language(&supported_languages))
        .or_else(|| get_browser_language(&supported_languages))
        .unwrap_or_else(|| Language::from_code("en"))
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

    // Initialize selected_language first
    let selected_language = {
        let language = determine_language(&props.config, &settings);
        SelectedLanguage::new(language.code())
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
            || ()
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

#[function_component(BrowserI18nProvider)]
pub fn browser_i18n_provider(props: &I18nProviderProps) -> Html {
    log::info!("🌐 BrowserI18nProvider initialization");

    // Start with the config from props
    let initial_config = props.config.clone();
    let supported_languages = initial_config.supported_languages();

    // Explicitly check URL parameters for debugging
    #[cfg(feature = "csr")]
    {
        use gloo::utils::window;
        use web_sys::UrlSearchParams;

        let window = window();
        if let Ok(search) = window.location().search() {
            log::info!("URL search string: {}", search);

            if !search.is_empty() {
                if let Ok(search_params) = UrlSearchParams::new_with_str(&search) {
                    if let Some(lang) = search_params.get("lang") {
                        log::info!("Found lang parameter: {}", lang);

                        if supported_languages.iter().any(|l| l.code() == lang) {
                            log::info!("Language '{}' is supported, will use it", lang);
                        } else {
                            log::warn!(
                                "Language '{}' is not in supported languages: {:?}",
                                lang,
                                supported_languages
                                    .iter()
                                    .map(|l| l.code())
                                    .collect::<Vec<_>>()
                            );
                        }
                    } else {
                        log::info!("No lang parameter found in URL");
                    }
                } else {
                    log::warn!("Could not parse URL search params");
                }
            }
        } else {
            log::warn!("Could not get search string from URL");
        }
    }

    // Determine the language with explicit logging
    let language = {
        #[cfg(feature = "csr")]
        {
            // Try URL path first
            if let Some(path_lang) = get_path_language(&supported_languages) {
                log::info!("Using language from URL path: {}", path_lang.code());
                path_lang
            }
            // Then URL query parameter
            else if let Some(url_lang) = get_url_language(&supported_languages) {
                log::info!("Using language from URL parameter: {}", url_lang.code());
                url_lang
            }
            // Then browser language
            else if let Some(browser_lang) = get_browser_language(&supported_languages) {
                log::info!(
                    "Using language from browser settings: {}",
                    browser_lang.code()
                );
                browser_lang
            }
            // Fallback to default (en)
            else {
                log::info!("Falling back to default language: en");
                Language::from_code("en")
            }
        }

        #[cfg(not(feature = "csr"))]
        {
            determine_browser_language(&initial_config, Some("en"))
        }
    };

    log::info!("Final selected language: {}", language.code());

    // Create selected language state
    let selected_language = SelectedLanguage::new(language.code());

    // Create config state and update default language if needed
    let config_ctx = use_state(|| {
        let mut config = initial_config.clone();
        if config.default_language.code() != language.code() {
            config.default_language = language.clone();
        }
        log::info!(
            "Config state initialized with language: {}",
            config.default_language.code()
        );
        config
    });

    // Effect to update language if URL or browser settings change
    {
        let config_ctx = config_ctx.clone();
        let initial_config = initial_config.clone();

        use_effect_with((), move |_| {
            #[cfg(feature = "csr")]
            {
                use gloo::events::EventListener;
                use gloo::utils::window;

                let window = window();

                // Immediate check for URL parameters
                if let Ok(search) = window.location().search() {
                    if !search.is_empty() {
                        if let Ok(search_params) = web_sys::UrlSearchParams::new_with_str(&search) {
                            if let Some(lang) = search_params.get("lang") {
                                log::info!("Effect: Found lang parameter: {}", lang);

                                let language = Language::from_code(&lang);
                                let mut new_config = initial_config.clone();

                                if new_config.default_language.code() != language.code() {
                                    log::info!("Effect: Updating language to {}", language.code());
                                    new_config.default_language = language;
                                    config_ctx.set(new_config);
                                }
                            }
                        }
                    }
                }

                // Create popstate event listener for navigation changes
                let listener = EventListener::new(&window, "popstate", move |_| {
                    log::info!("Popstate event detected, checking language");

                    let supported_langs = initial_config.supported_languages();

                    // Try URL path first
                    if let Some(path_lang) = get_path_language(&supported_langs) {
                        log::info!(
                            "Popstate: Using language from URL path: {}",
                            path_lang.code()
                        );
                        let mut new_config = initial_config.clone();
                        new_config.default_language = path_lang;
                        config_ctx.set(new_config);
                        return;
                    }

                    // Then URL query parameter
                    if let Some(url_lang) = get_url_language(&supported_langs) {
                        log::info!(
                            "Popstate: Using language from URL parameter: {}",
                            url_lang.code()
                        );
                        let mut new_config = initial_config.clone();
                        new_config.default_language = url_lang;
                        config_ctx.set(new_config);
                        return;
                    }

                    // Default fallback
                    log::info!("Popstate: No language in URL, using default");
                    let default_lang = Language::from_code("en");
                    let mut new_config = initial_config.clone();

                    if new_config.default_language.code() != default_lang.code() {
                        new_config.default_language = default_lang;
                        config_ctx.set(new_config);
                    }
                });

                // Keep listener alive until component is unmounted
                return move || {
                    drop(listener);
                };
            }

            #[cfg(not(feature = "csr"))]
            {
                || ()
            }
        });
    }

    // Use the existing I18nContext structure
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
