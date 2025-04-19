use crate::components::challenge::informative::InformativeComponentProps;
use konnektoren_core::asset_loader::AssetLoader;
use konnektoren_core::challenges::ChallengeResult;
use konnektoren_core::commands::{ChallengeCommand, Command};
use yew::prelude::*;

pub enum LoadingState {
    Loading,
    FetchSuccess(String),
    FetchError(String),
}

/// Function to load markdown content synchronously for SSG
#[cfg(feature = "ssr")]
pub fn load_markdown_for_ssg(path: &str) -> String {
    use std::path::PathBuf;

    // Get BUILD_DIR from environment variable at runtime or use defaults
    let build_dir = std::env::var("BUILD_DIR").unwrap_or_else(|_| "./".to_string());

    // Try multiple potential locations for the file
    let potential_paths = vec![
        PathBuf::from(path),                  // As provided
        PathBuf::from(&build_dir).join(path), // In build dir
        PathBuf::from("assets").join(path),   // In assets dir
    ];

    for file_path in potential_paths {
        if file_path.exists() {
            match std::fs::read_to_string(&file_path) {
                Ok(content) => return content,
                Err(e) => log::warn!("Failed to read file {}: {}", file_path.display(), e),
            }
        }
    }

    // Fallback content if file not found
    format!("*Failed to load markdown content from {}*", path)
}

#[function_component(InformativeMarkdownComponent)]
pub fn informative_markdown_component(props: &InformativeComponentProps) -> Html {
    let loading_state = use_state(|| LoadingState::Loading);
    let language = props.language.as_deref().unwrap_or("en");
    let asset_loader = use_state(AssetLoader::default);

    let on_finish = {
        let on_command = props.on_command.clone();
        Callback::from(move |_| {
            if let Some(on_command) = on_command.as_ref() {
                let command = Command::Challenge(ChallengeCommand::Finish(Some(
                    ChallengeResult::Informative,
                )));
                on_command.emit(command);
            }
        })
    };

    let scroll_to_bottom = {
        Callback::from(move |_| {
            #[cfg(feature = "csr")]
            {
                use wasm_bindgen::JsCast;
                use wasm_bindgen::prelude::*;
                use web_sys::HtmlElement;

                use gloo::timers::future::TimeoutFuture;

                wasm_bindgen_futures::spawn_local(async move {
                    // Wait a bit for the content to render
                    TimeoutFuture::new(100).await;
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(element) = document.get_element_by_id("finish-button") {
                                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                                    let rect = html_element.get_bounding_client_rect();
                                    let scroll_y = rect.top() + window.scroll_y().unwrap_or(0.0);
                                    scrollTo(0.0, scroll_y);
                                }
                            }
                        }
                    }
                });
            }
        })
    };

    // Get the markdown path
    let fallback_path = props
        .challenge
        .text
        .iter()
        .find(|t| t.language == "en")
        .cloned();
    let fallback_path = match fallback_path {
        Some(text) => text.text,
        None => {
            loading_state.set(LoadingState::FetchError("No text found".to_string()));
            "No text found".to_string()
        }
    };

    let informative_text = props
        .challenge
        .text
        .iter()
        .find(|t| t.language == language)
        .cloned();
    let markdown_path = match informative_text {
        Some(text) => text.text,
        None => fallback_path.clone(),
    };

    // SSG support: pre-load content during server-side rendering
    #[cfg(feature = "ssr")]
    {
        let loading_state_ssr = loading_state.clone();
        let markdown_path_ssr = markdown_path.clone();

        use_effect_with((), move |_| {
            let markdown_content = load_markdown_for_ssg(&markdown_path_ssr);
            loading_state_ssr.set(LoadingState::FetchSuccess(markdown_content));
            || ()
        });
    }

    // CSR support: load content asynchronously
    #[cfg(feature = "csr")]
    {
        let loading_state_csr = loading_state.clone();
        let markdown_path_csr = markdown_path.clone();
        let fallback_path_csr = fallback_path.clone();
        let asset_loader_csr = asset_loader.clone();

        use_effect_with((), move |_| {
            let markdown_path = markdown_path_csr.clone();
            let fallback_path = fallback_path_csr.clone();
            let loading_state = loading_state_csr.clone();
            let asset_loader = asset_loader_csr.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match fetch_markdown_with_loader(&asset_loader, &markdown_path).await {
                    Ok(content) => loading_state.set(LoadingState::FetchSuccess(content)),
                    Err(err) => {
                        log::warn!("Failed to fetch markdown {}: {}", markdown_path, err);
                        match fetch_markdown_with_loader(&asset_loader, &fallback_path).await {
                            Ok(content) => loading_state.set(LoadingState::FetchSuccess(content)),
                            Err(err) => loading_state.set(LoadingState::FetchError(err)),
                        }
                    }
                }
            });

            || ()
        });
    }

    match *loading_state {
        LoadingState::Loading => {
            html! {<p>{"Loading..."}</p>}
        }
        LoadingState::FetchError(ref error) => {
            html! {<p>{error}</p>}
        }
        LoadingState::FetchSuccess(ref text) => {
            let content = Html::from_html_unchecked(AttrValue::from(markdown::to_html(text)));
            html! {
                <div class="informative-markdown">
                    <h2>{&props.challenge.description}</h2>
                    <button onclick={scroll_to_bottom}>{"Scroll down"}</button>
                    <div class="markdown-content">
                        {content}
                    </div>
                    <button id="finish-button" onclick={on_finish}>{"Next"}</button>
                </div>
            }
        }
    }
}

#[cfg(feature = "csr")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "csr")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn scrollTo(x: f64, y: f64);
}

async fn fetch_markdown_with_loader(loader: &AssetLoader, path: &str) -> Result<String, String> {
    // Load the markdown file using the asset loader
    let binary_data = loader.load_binary(path).await?;

    // Convert bytes to string
    String::from_utf8(binary_data)
        .map_err(|e| format!("Failed to convert markdown content to string: {}", e))
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{Informative, InformativeText};
    use yew_preview::prelude::*;

    fn create_test_challenge() -> Informative {
        Informative {
            id: "".to_string(),
            name: "".to_string(),
            description: "Informative Challenge".to_string(),
            text: vec![InformativeText {
                language: "en".to_string(),
                text: "assets/articles.md".to_string(),
            }],
        }
    }

    yew_preview::create_preview!(
        InformativeMarkdownComponent,
        InformativeComponentProps {
            challenge: create_test_challenge(),
            on_command: None,
            language: None,
        },
        (
            "unknown language",
            InformativeComponentProps {
                challenge: create_test_challenge(),
                on_command: None,
                language: Some("de".to_string()),
            }
        ),
        (
            "unknown file",
            InformativeComponentProps {
                challenge: Informative {
                    id: "".to_string(),
                    name: "".to_string(),
                    description: "Informative Challenge".to_string(),
                    text: vec![InformativeText {
                        language: "en".to_string(),
                        text: "assets/unknown-en.md".to_string(),
                    }],
                },
                on_command: None,
                language: None,
            }
        )
    );
}
