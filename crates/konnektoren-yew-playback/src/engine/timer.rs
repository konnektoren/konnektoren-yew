use crate::model::PlaybackTiming;
use yew::prelude::*;

/// Schedules one step; changing the step, timing, callback or disabling it
/// cancels the previous timer. Browser timers are inactive during SSR.
#[hook]
pub fn use_playback_timer(timing: Option<PlaybackTiming>, step: usize, on_elapsed: Callback<()>) {
    let duration = timing.and_then(PlaybackTiming::duration_ms);
    #[cfg(feature = "csr")]
    {
        use_effect_with((duration, step, on_elapsed), |(duration, _, on_elapsed)| {
            let callback = on_elapsed.clone();
            let timer = duration.map(|duration| {
                gloo_timers::callback::Timeout::new(duration, move || callback.emit(()))
            });
            move || drop(timer)
        });
    }
    #[cfg(not(feature = "csr"))]
    {
        let _ = (duration, step, on_elapsed);
    }
}
