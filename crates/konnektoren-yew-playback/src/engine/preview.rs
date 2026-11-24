use super::{PlaybackEngine, PlaybackHost, PlaybackOutcome, PlaybackState, PlaybackToken};
use crate::{Excursion, PlaybackElementHighlight, PlaybackNotification, PlaybackStep};
use yew::prelude::*;
use yew_preview::prelude::*;

#[derive(Clone, PartialEq)]
struct PreviewState {
    engine: Option<PlaybackEngine>,
    message: Option<String>,
    highlight: Option<String>,
    zoom: f64,
    scroll: (f64, f64),
    error: Option<String>,
    autoplay: bool,
}

impl Default for PreviewState {
    fn default() -> Self {
        let parsed = serde_yaml::from_str::<Excursion>(include_str!("../../examples/preview.yaml"));
        match parsed {
            Ok(excursion) => Self {
                engine: Some(PlaybackEngine::new(excursion)),
                message: None,
                highlight: None,
                zoom: 1.0,
                scroll: (0.0, 0.0),
                error: None,
                autoplay: false,
            },
            Err(error) => Self {
                engine: None,
                message: None,
                highlight: None,
                zoom: 1.0,
                scroll: (0.0, 0.0),
                error: Some(error.to_string()),
                autoplay: false,
            },
        }
    }
}

struct PreviewHost<'a>(&'a mut PreviewState);

impl PlaybackHost for PreviewHost<'_> {
    type Error = &'static str;

    fn execute(
        &mut self,
        step: &PlaybackStep,
        _: PlaybackToken,
    ) -> Result<PlaybackOutcome, Self::Error> {
        match step {
            PlaybackStep::Say { say } => self.0.message = Some(say.clone()),
            PlaybackStep::Highlight { highlight } => self.0.highlight = Some(highlight.clone()),
            PlaybackStep::Scroll { scroll } => self.0.scroll = (scroll.x, scroll.y),
            PlaybackStep::Zoom { zoom } if zoom.is_finite() && *zoom > 0.0 => self.0.zoom = *zoom,
            _ => return Err("This action is not supported by the preview host."),
        }
        Ok(PlaybackOutcome::Complete)
    }
}

#[function_component(PlaybackEnginePreview)]
pub fn playback_engine_preview() -> Html {
    let state = use_state(PreviewState::default);
    let viewport = use_node_ref();
    let advance = {
        let state = state.clone();
        Callback::from(move |()| {
            let mut next = (*state).clone();
            if let Some(mut engine) = next.engine.take() {
                if let Err(error) = engine.advance(&mut PreviewHost(&mut next)) {
                    next.error = Some(error.into());
                    next.autoplay = false;
                }
                if engine.state() == PlaybackState::Finished {
                    next.autoplay = false;
                }
                next.engine = Some(engine);
            }
            state.set(next);
        })
    };
    #[cfg(feature = "csr")]
    {
        let advance = advance.clone();
        let step = state.engine.as_ref().map(PlaybackEngine::index);
        use_effect_with((state.autoplay, step), move |(autoplay, _)| {
            let timer = autoplay
                .then(|| gloo_timers::callback::Timeout::new(1600, move || advance.emit(())));
            move || drop(timer)
        });
    }
    #[cfg(feature = "csr")]
    {
        let viewport = viewport.clone();
        use_effect_with(state.scroll, move |(x, y)| {
            if let Some(element) = viewport.cast::<web_sys::HtmlElement>() {
                element.set_scroll_left(*x as i32);
                element.set_scroll_top(*y as i32);
            }
            || ()
        });
    }
    let toggle = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut next = (*state).clone();
            next.autoplay = !next.autoplay;
            state.set(next);
        })
    };
    let reset = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| state.set(PreviewState::default()))
    };
    let done = state
        .engine
        .as_ref()
        .is_none_or(|engine| engine.state() == PlaybackState::Finished);
    let position = state.engine.as_ref().map_or_else(
        || "Unavailable".to_string(),
        |engine| format!("{} / {}", engine.index(), engine.total()),
    );
    html! {
        <div class="playback-engine-preview">
            <div class="playback-engine-preview__controls">
                <button type="button" class="playback-engine-preview__button" onclick={toggle} disabled={done}>{if state.autoplay { "Pause" } else { "Play" }}</button>
                <button type="button" class="playback-engine-preview__button" onclick={advance.reform(|_: MouseEvent| ())} disabled={done || state.autoplay}>{"Next step"}</button>
                <button type="button" class="playback-engine-preview__button" onclick={reset}>{"Restart"}</button>
                <span>{format!("Step {position}")}</span>
            </div>
            if let Some(error) = &state.error { <p role="alert">{error}</p> }
            if let Some(message) = &state.message { <PlaybackNotification message={message.clone()} /> }
            <div ref={viewport} class="playback-engine-preview__viewport">
                <div class="playback-engine-preview__page" style={format!("zoom:{}", state.zoom)}>
                    <h3 class="playback-engine-preview__header">{"Example page"}</h3>
                    <p>{"An excursion can introduce a page, then move through its content."}</p>
                    <div class="playback-engine-preview__spacer" aria-hidden="true"></div>
                    <ul class="playback-engine-preview__list">
                        <li>{"First item"}</li><li>{"Second item"}</li><li>{"Third item"}</li>
                    </ul>
                </div>
            </div>
            if let Some(selector) = &state.highlight { <PlaybackElementHighlight selector={selector.clone()} /> }
        </div>
    }
}

yew_preview::create_preview!(PlaybackEnginePreview, (),);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_excursion_runs_all_supported_actions() {
        let mut state = PreviewState::default();
        let mut engine = state.engine.take().expect("preview YAML should parse");
        while engine.state() == PlaybackState::Playing {
            engine
                .advance(&mut PreviewHost(&mut state))
                .expect("preview action should succeed");
        }
        assert_eq!(state.scroll, (0.0, 260.0));
        assert_eq!(state.zoom, 1.2);
        assert_eq!(
            state.highlight.as_deref(),
            Some(".playback-engine-preview__list")
        );
        assert_eq!(state.message.as_deref(), Some("The page is now larger."));
    }
}
