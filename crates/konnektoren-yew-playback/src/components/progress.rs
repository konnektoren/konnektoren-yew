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
    #[prop_or_default]
    pub class: Classes,
    /// Only show for a few seconds after the user clicks or presses a key,
    /// e.g. to keep a recorded or streamed excursion clean.
    #[prop_or_default]
    pub hide_when_idle: bool,
}

#[cfg(feature = "csr")]
const IDLE_MS: u32 = 5_000;

/// Displays the current playback position and speed.
#[function_component(PlaybackProgress)]
pub fn playback_progress(props: &PlaybackProgressProps) -> Html {
    let recent_input = use_recent_input(props.hide_when_idle);
    if props.hide_when_idle && !recent_input {
        return Html::default();
    }
    html! {
        <aside class={classes!("playback-progress", props.class.clone())} aria-label={props.controls_label.clone()}>
            <span class="playback-progress__step">{format!("{} {} / {}", props.step_label, props.step, props.total)}</span>
            <span class="playback-progress__speed">{format!("{} {:.1}×", props.speed_label, props.speed)}</span>
            if let Some(label) = &props.controls_label {
                <span class="playback-progress__controls">{label.clone()}</span>
            }
        </aside>
    }
}

/// Whether the user clicked or pressed a key within the last few seconds.
#[hook]
fn use_recent_input(enabled: bool) -> bool {
    let last_input = use_state_eq(|| None::<u64>);
    #[cfg(feature = "csr")]
    {
        let sequence = use_mut_ref(|| 0_u64);
        {
            let last_input = last_input.clone();
            use_effect_with(enabled, move |enabled| {
                let listeners = enabled.then(web_sys::window).flatten().map(|window| {
                    ["click", "keydown"].map(|kind| {
                        let last_input = last_input.clone();
                        let sequence = sequence.clone();
                        gloo_events::EventListener::new(&window, kind, move |event| {
                            if event.is_trusted() {
                                let mut count = sequence.borrow_mut();
                                *count = count.wrapping_add(1);
                                last_input.set(Some(*count));
                            }
                        })
                    })
                });
                move || drop(listeners)
            });
        }
        let last_input = last_input.clone();
        use_effect_with(*last_input, move |input| {
            let timeout = input.map(|_| {
                gloo_timers::callback::Timeout::new(IDLE_MS, move || last_input.set(None))
            });
            move || drop(timeout)
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = enabled;
    last_input.is_some()
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
            class: Classes::new(),
            hide_when_idle: false,
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
                class: Classes::new(),
                hide_when_idle: false,
            }
        ),
    );
}
