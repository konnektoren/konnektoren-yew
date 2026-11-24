use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackClickWaveProps {
    /// The click position in viewport pixels.
    pub x: f64,
    pub y: f64,
}

/// A decorative, non-interactive click effect.
#[function_component(PlaybackClickWave)]
pub fn playback_click_wave(props: &PlaybackClickWaveProps) -> Html {
    let style = format!("left:{}px;top:{}px", props.x, props.y);
    html! { <div class="playback-click-wave" {style} aria-hidden="true" /> }
}

#[cfg(feature = "yew-preview")]
#[function_component(PlaybackClickWavePreview)]
pub fn playback_click_wave_preview() -> Html {
    let effect = use_state(|| None::<(u32, f64, f64)>);
    let onclick = {
        let effect = effect.clone();
        Callback::from(move |event: MouseEvent| {
            let next = effect.as_ref().map_or(0, |(id, _, _)| id.wrapping_add(1));
            effect.set(Some((
                next,
                f64::from(event.client_x()),
                f64::from(event.client_y()),
            )));
        })
    };
    html! {
        <>
            <button type="button" {onclick}>{"Click to replay the effect"}</button>
            if let Some((id, x, y)) = *effect {
                <PlaybackClickWave key={id.to_string()} {x} {y} />
            }
        </>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(PlaybackClickWavePreview, (),);
}
