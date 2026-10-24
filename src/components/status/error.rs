use crate::i18n::use_i18n;
use yew::prelude::*;

fn error_icon() -> Html {
    html! { <span>{"⚠"}</span> }
}

#[derive(Properties, PartialEq)]
pub struct ErrorStateProps {
    pub message: AttrValue,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_else(error_icon)]
    pub icon: Html,
    /// The button is shown only when a retry action is available.
    #[prop_or_default]
    pub on_retry: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub retry_label: Option<AttrValue>,
    /// Additional explanation or actions, such as a download or back link.
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ErrorState)]
pub fn error_state(props: &ErrorStateProps) -> Html {
    let i18n = use_i18n();
    html! {
        <div class="status-panel error-state" role="alert">
            if props.icon != Html::default() {
                <div class="status-panel__icon" aria-hidden="true">{props.icon.clone()}</div>
            }
            if let Some(title) = &props.title { <h3 class="status-panel__title">{title}</h3> }
            <p class="status-panel__message">{props.message.clone()}</p>
            if let Some(on_retry) = &props.on_retry {
                <button type="button" class="status-panel__action btn btn-outline" onclick={on_retry.clone()}>
                    {props.retry_label.clone().unwrap_or_else(|| i18n.t("Try again").into())}
                </button>
            }
            {for props.children.iter()}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn props() -> ErrorStateProps {
        ErrorStateProps {
            message: "Could not load exercises.".into(),
            title: None,
            icon: error_icon(),
            on_retry: None,
            retry_label: None,
            children: Children::default(),
        }
    }

    yew_preview::create_preview!(
        ErrorState,
        props(),
        (
            "with retry",
            ErrorStateProps {
                title: Some("Exercises unavailable".into()),
                on_retry: Some(Callback::noop()),
                ..props()
            }
        ),
        (
            "custom action",
            ErrorStateProps {
                children: Children::new(vec![
                    html! { <a class="btn btn-ghost" href="/">{"Back to home"}</a> }
                ]),
                ..props()
            }
        ),
    );
}
