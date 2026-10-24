use yew::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum LoadingIndicator {
    #[default]
    Spinner,
    Skeleton,
    None,
}

#[derive(Properties, PartialEq)]
pub struct LoadingStateProps {
    pub message: AttrValue,
    #[prop_or_default]
    pub indicator: LoadingIndicator,
    /// Optional decorative placeholder, replacing the built-in indicator.
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoadingState)]
pub fn loading_state(props: &LoadingStateProps) -> Html {
    let placeholder = if !props.children.is_empty() {
        html! { <div class="loading-state__placeholder" aria-hidden="true">{for props.children.iter()}</div> }
    } else {
        match props.indicator {
            LoadingIndicator::Spinner => {
                html! { <span class="loading-state__spinner" aria-hidden="true" /> }
            }
            LoadingIndicator::Skeleton => html! {
                <div class="loading-state__skeletons" aria-hidden="true">
                    <div class="loading-state__skeleton" />
                    <div class="loading-state__skeleton" />
                    <div class="loading-state__skeleton" />
                </div>
            },
            LoadingIndicator::None => Html::default(),
        }
    };
    html! {
        <div class="status-panel loading-state" role="status">
            {placeholder}
            <p class="status-panel__message">{props.message.clone()}</p>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn props(indicator: LoadingIndicator) -> LoadingStateProps {
        LoadingStateProps {
            message: "Loading exercises…".into(),
            indicator,
            children: Children::default(),
        }
    }

    yew_preview::create_preview!(
        LoadingState,
        props(LoadingIndicator::Spinner),
        ("skeleton", props(LoadingIndicator::Skeleton)),
        ("text only", props(LoadingIndicator::None)),
        (
            "custom placeholder",
            LoadingStateProps {
                children: Children::new(vec![html! { <div class="loading-state__skeleton" /> }]),
                ..props(LoadingIndicator::None)
            }
        ),
    );
}
