use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackProgressProps {
    /// Current step, counted from one.
    pub step: usize,
    pub total: usize,
    pub speed: f64,
    /// Labels are supplied translated by the host.
    pub step_label: AttrValue,
    pub speed_label: AttrValue,
    #[prop_or_default]
    pub controls_label: Option<AttrValue>,
}

/// Displays the current playback position and speed.
#[function_component(PlaybackProgress)]
pub fn playback_progress(props: &PlaybackProgressProps) -> Html {
    html! {
        <aside class="playback-progress" aria-label={props.controls_label.clone()}>
            <span class="playback-progress__step">{format!("{} {} / {}", props.step_label, props.step, props.total)}</span>
            <span class="playback-progress__speed">{format!("{} {:.1}×", props.speed_label, props.speed)}</span>
            if let Some(label) = &props.controls_label {
                <span class="playback-progress__controls">{label.clone()}</span>
            }
        </aside>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackProgress,
        PlaybackProgressProps {
            step: 3,
            total: 12,
            speed: 1.0,
            step_label: "Step".into(),
            speed_label: "Speed".into(),
            controls_label: Some("← slower · → faster · ↑ next".into()),
        },
        (
            "Faster",
            PlaybackProgressProps {
                step: 8,
                total: 12,
                speed: 2.0,
                step_label: "Step".into(),
                speed_label: "Speed".into(),
                controls_label: None,
            }
        ),
    );
}
