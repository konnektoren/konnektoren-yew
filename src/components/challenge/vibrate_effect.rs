use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VibrateEffectProps {
    pub duration: u32,
}

/// Vibrates the device (Chrome on Android only) once when rendered.
#[function_component(VibrateEffectComponent)]
pub fn vibrate_effect(props: &VibrateEffectProps) -> Html {
    #[cfg(feature = "csr")]
    {
        let duration = props.duration;

        use_effect(move || {
            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                if let Ok(user_agent) = navigator.user_agent() {
                    if user_agent.contains("Android") && user_agent.contains("Chrome") {
                        if !navigator.vibrate_with_duration(duration) {
                            tracing::error!("Vibration effect failed");
                        }
                    } else {
                        tracing::debug!("Not Chrome on Android, skipping vibration");
                    }
                }
            }
            || ()
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = props;

    html! {}
}
