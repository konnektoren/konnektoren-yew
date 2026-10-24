use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmptyStateProps {
    pub message: AttrValue,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub icon: Html,
    /// Optional action content, supplied by the caller.
    #[prop_or_default]
    pub children: Children,
}

#[function_component(EmptyState)]
pub fn empty_state(props: &EmptyStateProps) -> Html {
    html! {
        <div class="status-panel empty-state" role="status">
            if props.icon != Html::default() {
                <div class="status-panel__icon" aria-hidden="true">{props.icon.clone()}</div>
            }
            if let Some(title) = &props.title { <h3 class="status-panel__title">{title}</h3> }
            <p class="status-panel__message">{props.message.clone()}</p>
            {for props.children.iter()}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn props() -> EmptyStateProps {
        EmptyStateProps {
            message: "No results found.".into(),
            title: None,
            icon: Html::default(),
            children: Children::default(),
        }
    }

    yew_preview::create_preview!(
        EmptyState,
        props(),
        (
            "with icon",
            EmptyStateProps {
                title: Some("No recordings yet".into()),
                icon: html! { <span>{"🎵"}</span> },
                ..props()
            }
        ),
        (
            "with action",
            EmptyStateProps {
                children: Children::new(vec![
                    html! { <a href="/" class="btn btn-primary">{"Explore exercises"}</a> }
                ]),
                ..props()
            }
        ),
    );
}
