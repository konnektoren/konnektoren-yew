use crate::components::TimerComponent;
use chrono::Duration;
use konnektoren_core::challenges::{Challenge, Timed};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct ChallengeTimerProps {
    pub challenge: Challenge,
    #[prop_or_default]
    pub running: bool,
    #[prop_or_default]
    pub show_milliseconds: bool,
}

#[function_component(ChallengeTimerComponent)]
pub fn challenge_timer_component(props: &ChallengeTimerProps) -> Html {
    let running = props.running;
    let challenge = props.challenge.clone();
    let show_milliseconds = props.show_milliseconds;
    let duration = use_state(|| get_duration(props.running, &props.challenge));

    {
        let duration = duration.clone();
        use_effect_with(
            (props.running, props.challenge.clone(), show_milliseconds),
            move |_| {
                let running = running;
                let challenge = challenge;
                let duration = duration.clone();
                let show_milliseconds = show_milliseconds;
                let timeout = match show_milliseconds {
                    true => 142,
                    false => 1000,
                };

                if running {
                    let interval = gloo::timers::callback::Interval::new(timeout, move || {
                        duration.set(get_duration(running, &challenge));
                    });
                    interval.forget();
                }

                || {}
            },
        );
    }

    html! {
        <div class="challenge-timer">
        <TimerComponent
            milliseconds={duration.map(|d| d.num_milliseconds()).unwrap_or_default()}
            show_milliseconds={show_milliseconds}
        />
        </div>
    }
}

fn get_duration(running: bool, challenge: &Challenge) -> Option<Duration> {
    match running {
        true => {
            let end = chrono::Utc::now();
            let start = challenge.start_time().unwrap_or(end);
            Some(end - start)
        }
        false => challenge.elapsed_time(),
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeTimerComponent,
        ChallengeTimerProps::default(),
        (
            "not running",
            ChallengeTimerProps {
                challenge: Challenge::default(),
                running: false,
                show_milliseconds: false,
            }
        ),
        (
            "running",
            ChallengeTimerProps {
                challenge: Challenge {
                    start_time: Some(chrono::Utc::now()),
                    end_time: None,
                    ..Challenge::default()
                },
                running: true,
                ..ChallengeTimerProps::default()
            }
        ),
        (
            "running with milliseconds",
            ChallengeTimerProps {
                challenge: Challenge {
                    start_time: Some(chrono::Utc::now()),
                    end_time: None,
                    ..Challenge::default()
                },
                running: true,
                show_milliseconds: true,
            }
        )
    );
}
