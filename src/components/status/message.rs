use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum StatusType {
    Text(String),
    Info(String),
    Warning(String),
    Error(String),
}

impl Default for StatusType {
    fn default() -> Self {
        StatusType::Text("This is a text message".to_string())
    }
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct StatusMessageProps {
    pub status: StatusType,
    #[prop_or_default]
    pub title: Option<String>,
}

#[function_component(StatusMessage)]
pub fn status_message(props: &StatusMessageProps) -> Html {
    let (content, modifier) = match &props.status {
        StatusType::Text(text) => (text, "text"),
        StatusType::Info(text) => (text, "info"),
        StatusType::Warning(text) => (text, "warning"),
        StatusType::Error(text) => (text, "error"),
    };

    let role = match &props.status {
        StatusType::Error(_) => Some("alert"),
        StatusType::Info(_) | StatusType::Warning(_) => Some("status"),
        StatusType::Text(_) => None,
    };

    html! {
        <div class={classes!("status-panel", "status-message", format!("status-message--{}", modifier))} {role}>
            if let Some(title) = &props.title {
                <h3 class="status-message__title">{ title }</h3>
            }
            <p class="status-message__content">{ content }</p>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        StatusMessage,
        StatusMessageProps::default(),
        (
            "Text",
            StatusMessageProps {
                status: StatusType::Text("This is a text message".to_string()),
                title: Some("Text".to_string())
            }
        ),
        (
            "Info",
            StatusMessageProps {
                status: StatusType::Info("This is an info message".to_string()),
                title: Some("Info".to_string())
            }
        ),
        (
            "Warning",
            StatusMessageProps {
                status: StatusType::Warning("This is a warning message".to_string()),
                title: Some("Warning".to_string())
            }
        ),
        (
            "Error",
            StatusMessageProps {
                status: StatusType::Error("This is an error message".to_string()),
                title: Some("Error".to_string())
            }
        ),
    );
}
