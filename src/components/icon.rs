use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub icon: String,
    pub alt: String,
    /// Base class applied to the rendered icon; text icons also get `--text`.
    pub class: String,
    /// Optional class for images when they need different styling from font icons.
    #[prop_or_default]
    pub image_class: Option<String>,
    /// Treat non-Font-Awesome values as image paths, including relative paths.
    #[prop_or(false)]
    pub fallback_as_image: bool,
}

/// Render a Font Awesome icon, image URL, or text such as an emoji.
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    if props.icon.starts_with("fa-") {
        html! { <i class={classes!(props.class.clone(), "fas", props.icon.clone())} aria-hidden="true"></i> }
    } else if props.fallback_as_image
        || props.icon.starts_with("http")
        || props.icon.starts_with("data:")
    {
        html! { <img class={props.image_class.clone().unwrap_or_else(|| props.class.clone())} src={props.icon.clone()} alt={props.alt.clone()} /> }
    } else {
        html! { <span class={classes!(props.class.clone(), format!("{}--text", props.class))}>{&props.icon}</span> }
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        Icon,
        IconProps {
            icon: "fa-star".into(),
            alt: "Star".into(),
            class: "achievement__icon".into(),
            image_class: None,
            fallback_as_image: false,
        },
    );
}
