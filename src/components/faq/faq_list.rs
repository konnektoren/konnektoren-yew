use super::FaqItem;
use konnektoren_asset_loader::AssetServer;
#[cfg(feature = "csr")]
use konnektoren_core::assets::AssetFormat;
use konnektoren_core::assets::DefaultAssetSource;
use konnektoren_rs::faq::FaqData;
use std::rc::Rc;
use yew::prelude::*;

#[cfg(feature = "csr")]
const FAQ_ASSET_PATH: &str = "faqs.yml";

#[cfg(feature = "csr")]
#[derive(Clone)]
struct FaqFormat;

#[cfg(feature = "csr")]
impl AssetFormat for FaqFormat {
    type Asset = FaqData;
    type Error = serde_yaml::Error;

    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }

    fn parse(&self, bytes: &[u8]) -> Result<Self::Asset, Self::Error> {
        serde_yaml::from_slice(bytes)
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct FaqListProps {
    pub faq_data: FaqData,
    #[prop_or_else(|| "en".to_string())]
    pub language: String,
    #[prop_or_default]
    pub highlighted_id: Option<String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct FaqLoaderProps {
    #[prop_or_else(|| "en".to_string())]
    pub language: String,
    #[prop_or_default]
    pub highlighted_id: Option<String>,
}

/// Loads the FAQ asset through the configured asset source and renders it.
///
/// `FaqList` remains available for callers that already have parsed data,
/// while this component is the runtime-loading entry point.
#[function_component(FaqLoader)]
pub fn faq_loader(props: &FaqLoaderProps) -> Html {
    let faq_data = use_state(|| None::<FaqData>);
    let load_error = use_state(|| None::<String>);
    let asset_server = use_state(|| AssetServer::new(Rc::new(DefaultAssetSource::default())));

    {
        let faq_data = faq_data.clone();
        let load_error = load_error.clone();
        let asset_server = asset_server.clone();
        use_effect_with((), move |_| {
            #[cfg(feature = "csr")]
            {
                wasm_bindgen_futures::spawn_local(async move {
                    match asset_server.load(FaqFormat, FAQ_ASSET_PATH).await {
                        Ok(data) => faq_data.set(Some((*data).clone())),
                        Err(error) => load_error.set(Some(error.to_string())),
                    }
                });
            }
            || ()
        });
    }

    if let Some(error) = (*load_error).as_deref() {
        return html! { <p>{format!("Failed to load FAQ: {error}")}</p> };
    }

    match (*faq_data).clone() {
        Some(faq_data) => html! {
            <FaqList
                faq_data={faq_data}
                language={props.language.clone()}
                highlighted_id={props.highlighted_id.clone()}
            />
        },
        None => html! { <p>{"Loading FAQ..."}</p> },
    }
}

#[function_component(FaqList)]
pub fn faq_list(props: &FaqListProps) -> Html {
    html! {
        <section class="faq-list">
            {for props.faq_data.faqs.iter().cloned().map(|faq| {
                let highlighted = props.highlighted_id.as_deref() == Some(faq.id.as_str());
                html! {
                    <FaqItem
                        faq={faq}
                        language={props.language.clone()}
                        highlighted={highlighted}
                    />
                }
            })}
        </section>
    }
}

#[cfg(feature = "yew-preview")]
mod loader_preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        FaqLoader,
        FaqLoaderProps {
            language: "de".to_string(),
            highlighted_id: None,
        },
        (
            "highlighted FAQ",
            FaqLoaderProps {
                language: "en".to_string(),
                highlighted_id: Some("general-what-is-konnektoren".to_string()),
            }
        )
    );
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn preview_faq_data() -> FaqData {
        serde_yaml::from_str(include_str!("../../../assets/faqs.yml")).unwrap_or_default()
    }

    yew_preview::create_preview!(
        FaqList,
        FaqListProps {
            faq_data: preview_faq_data(),
            language: "de".to_string(),
            highlighted_id: Some("general-what-is-konnektoren".to_string()),
        },
        (
            "highlighted theme",
            FaqListProps {
                faq_data: preview_faq_data(),
                language: "de".to_string(),
                highlighted_id: Some("general-theme-switch".to_string()),
            }
        ),
        (
            "not highlighted",
            FaqListProps {
                faq_data: preview_faq_data(),
                language: "de".to_string(),
                highlighted_id: None,
            }
        ),
        (
            "english highlighted",
            FaqListProps {
                faq_data: preview_faq_data(),
                language: "en".to_string(),
                highlighted_id: Some("general-theme-switch".to_string()),
            }
        ),
    );
}
