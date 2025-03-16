use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub value: usize,
    pub max: Option<usize>,
    pub label: String,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    html! {
        <div class="progress-bar">
            <progress
                class="progress-bar__fill"
                value={props.value.to_string()}
                max={props.max.unwrap_or(100).to_string()}
            >
                <span class="progress-bar__label">{&props.label}</span>
            </progress>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ProgressBar,
        ProgressBarProps {
            value: 50,
            max: Some(100),
            label: "50%".to_string()
        },
        (
            "75%",
            ProgressBarProps {
                value: 75,
                max: Some(100),
                label: "75%".to_string()
            }
        )
    );
}
