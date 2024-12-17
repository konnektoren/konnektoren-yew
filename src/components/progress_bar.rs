use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub value: usize,
    pub max: Option<usize>,
    pub label: String,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let progress = match props.max {
        Some(max) => (props.value as f64 / max as f64) * 100.0,
        None => props.value as f64,
    };

    html! {
        <div class="progress">
            <div
                class="progress__bar"
                style={format!("width: {:.2}%;", progress)}
            >
                <span class="progress__label">
                    {props.label.clone()}
                </span>
            </div>
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
