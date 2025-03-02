use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub label: String,
    pub description: String,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    html! {
        <div class="badge__content">
            <span class="badge__label">{ &props.label }</span>
            <div class="badge__tooltip">
                { &props.description }
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        Badge,
        BadgeProps {
            label: "Badge".to_string(),
            description: "This is a badge".to_string(),
        },
    );
}
