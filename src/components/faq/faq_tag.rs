use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FaqTagProps {
    pub tag: String,
}

#[function_component(FaqTag)]
pub fn faq_tag(props: &FaqTagProps) -> Html {
    html! {
        <span class="faq-tag">{&props.tag}</span>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        FaqTag,
        FaqTagProps {
            tag: "general".to_string(),
        },
    );
}
