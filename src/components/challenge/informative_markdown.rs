use crate::components::challenge::informative::InformativeComponentProps;
use gloo::net::http::Request;
use konnektoren_core::challenges::ChallengeResult;
use konnektoren_core::commands::{ChallengeCommand, Command};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

pub enum LoadingState {
    Loading,
    FetchSuccess(String),
    FetchError(String),
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn scrollTo(x: f64, y: f64);
}

#[function_component(InformativeMarkdownComponent)]
pub fn informative_markdown_component(props: &InformativeComponentProps) -> Html {
    let loading_state = use_state(|| LoadingState::Loading);
    let language = props.language.as_deref().unwrap_or("en");

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

    let scroll_to_bottom = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            // Wait a bit for the content to render
            gloo::timers::future::TimeoutFuture::new(100).await;
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
    });

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
        None => fallback_path.to_string(),
    };

    {
        let loading_state = loading_state.clone();
        use_effect_with((), move |_| {
            let markdown_path = markdown_path.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_markdown(&markdown_path).await {
                    Ok(content) => loading_state.set(LoadingState::FetchSuccess(content)),
                    Err(_) => match fetch_markdown(&fallback_path).await {
                        Ok(content) => loading_state.set(LoadingState::FetchSuccess(content)),
                        Err(err) => loading_state.set(LoadingState::FetchError(err)),
                    },
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

async fn fetch_markdown(path: &str) -> Result<String, String> {
    let response = Request::get(path)
        .send()
        .await
        .map_err(|_| format!("Failed to fetch the file {}", path))?;
    if response.status() == 200 {
        response
            .text()
            .await
            .map_err(|_| format!("Failed to read the file content of {}", path))
    } else {
        Err(format!("File not found: {}", path))
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{Informative, InformativeText};
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        InformativeMarkdownComponent,
        InformativeComponentProps {
            challenge: Informative {
                id: "".to_string(),
                name: "".to_string(),
                description: "Informative Challenge".to_string(),
                text: vec![InformativeText {
                    language: "en".to_string(),
                    text: "assets/articles.md".to_string(),
                }],
            },
            on_command: None,
            language: None,
        },
        (
            "unknown language",
            InformativeComponentProps {
                challenge: Informative {
                    id: "".to_string(),
                    name: "".to_string(),
                    description: "Informative Challenge".to_string(),
                    text: vec![InformativeText {
                        language: "en".to_string(),
                        text: "assets/articles-de.md".to_string(),
                    }],
                },
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
