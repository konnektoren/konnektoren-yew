use super::FaqList;
use crate::asset::{FAQ_ASSET_PATH, FaqDataState, use_faq_data};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FaqLoaderProps {
    #[prop_or_else(|| "en".to_string())]
    pub language: String,
    #[prop_or_default]
    pub highlighted_id: Option<String>,
    /// FAQ file relative to the asset root.
    #[prop_or_else(|| FAQ_ASSET_PATH.into())]
    pub path: AttrValue,
}

/// Loads the FAQ asset through the asset loader and renders it.
///
/// `FaqList` remains available for callers that already have parsed data,
/// while this component is the runtime-loading entry point.
#[function_component(FaqLoader)]
pub fn faq_loader(props: &FaqLoaderProps) -> Html {
    match use_faq_data(props.path.clone()) {
        FaqDataState::Loaded(faq_data) => html! {
            <FaqList
                faq_data={(*faq_data).clone()}
                language={props.language.clone()}
                highlighted_id={props.highlighted_id.clone()}
            />
        },
        FaqDataState::Failed(error) => html! { <p>{format!("Failed to load FAQ: {error}")}</p> },
        FaqDataState::Loading => html! { <p>{"Loading FAQ..."}</p> },
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        FaqLoader,
        FaqLoaderProps {
            language: "de".to_string(),
            highlighted_id: None,
            path: FAQ_ASSET_PATH.into(),
        },
        (
            "highlighted FAQ",
            FaqLoaderProps {
                language: "en".to_string(),
                highlighted_id: Some("general-what-is-konnektoren".to_string()),
                path: FAQ_ASSET_PATH.into(),
            }
        )
    );
}
