use super::{PlaybackOptions, PlaybackState, use_playback, use_playback_keys};
use crate::{
    Excursion, PlaybackClickWave, PlaybackCursor, PlaybackHighlight, PlaybackNotification,
    PlaybackProgress,
};
use std::rc::Rc;
use yew::prelude::*;
use yew_preview::prelude::*;

fn preview_excursion() -> Result<Rc<Excursion>, String> {
    serde_yaml::from_str::<Excursion>(include_str!("../../examples/preview.yaml"))
        .map(Rc::new)
        .map_err(|error| error.to_string())
}

#[derive(Properties, PartialEq)]
struct PreviewRunProps {
    excursion: Rc<Excursion>,
}

#[function_component(PreviewRun)]
fn preview_run(props: &PreviewRunProps) -> Html {
    let playback = use_playback(
        Some(props.excursion.clone()),
        PlaybackOptions {
            step_delay_ms: 2_000,
            ..PlaybackOptions::default()
        },
        (),
    );
    use_playback_keys(playback.controls.clone());
    let paused = playback.state == PlaybackState::Paused;
    let toggle = {
        let controls = playback.controls.clone();
        Callback::from(move |_: MouseEvent| {
            if paused {
                controls.resume()
            } else {
                controls.pause()
            }
        })
    };
    let next = {
        let controls = playback.controls.clone();
        Callback::from(move |_: MouseEvent| controls.next())
    };
    let done = playback.state == PlaybackState::Finished;
    let clicked = use_state(|| false);
    html! {
        <div class="playback-engine-preview">
            <div class="playback-engine-preview__controls">
                <button type="button" class="playback-engine-preview__button" onclick={toggle} disabled={done}>{if paused { "Play" } else { "Pause" }}</button>
                <button type="button" class="playback-engine-preview__button" onclick={next} disabled={done}>{"Next step"}</button>
                <PlaybackProgress step={playback.index + 1} total={playback.total} speed={playback.speed}
                    step_label="Step" speed_label="Speed" controls_label={Some(AttrValue::from("← slower · → faster · ↑ next"))} />
            </div>
            if let Some(step) = playback.step() {
                <PlaybackNotification key={playback.step_key()} message={step.say.clone()} speak={step.speak} position={step.position} lang="en" />
            }
            <div class="playback-engine-preview__viewport">
                <div class="playback-engine-preview__page">
                    <h3 class="playback-engine-preview__header">{"Example page"}</h3>
                    <p>{"An excursion can introduce a page, then move through its content."}</p>
                    <ul class="playback-engine-preview__list">
                        <li>{"First item"}</li><li>{"Second item"}</li><li>{"Third item"}</li>
                    </ul>
                    <input class="playback-engine-preview__input" placeholder="Name" />
                    <button type="button" class="playback-engine-preview__button-target"
                        onclick={let clicked = clicked.clone(); move |_| clicked.set(true)}>
                        {if *clicked { "Clicked!" } else { "Button" }}
                    </button>
                </div>
            </div>
            if let Some(rect) = playback.highlight() { <PlaybackHighlight {rect} /> }
            if let Some(click) = playback.click { <PlaybackClickWave key={click.id} x={click.x} y={click.y} /> }
            if let Some((x, y)) = playback.cursor { <PlaybackCursor {x} {y} /> }
        </div>
    }
}

#[function_component(PlaybackEnginePreview)]
pub fn playback_engine_preview() -> Html {
    let run = use_state(|| 0_u32);
    let restart = {
        let run = run.clone();
        Callback::from(move |_: MouseEvent| run.set(run.wrapping_add(1)))
    };
    match preview_excursion() {
        Ok(excursion) => html! {
            <>
                <button type="button" class="playback-engine-preview__button" onclick={restart}>{"Restart"}</button>
                <PreviewRun key={*run} {excursion} />
            </>
        },
        Err(error) => html! { <p role="alert">{error}</p> },
    }
}

yew_preview::create_preview!(PlaybackEnginePreview, (),);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_excursion_parses() {
        let excursion = preview_excursion().expect("preview YAML should parse");
        assert!(excursion.steps.iter().any(|step| step.action.is_some()));
    }
}
