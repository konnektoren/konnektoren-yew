use chrono::Duration;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TimerProps {
    pub milliseconds: i64,
    #[prop_or_default]
    pub show_milliseconds: bool,
}

#[function_component(TimerComponent)]
pub fn timer_component(props: &TimerProps) -> Html {
    let duration = Duration::milliseconds(props.milliseconds);

    html! {
        <div class="timer">
            <div class="timer__icon">
                <i class="fa-solid fa-clock"></i>
            </div>
            <div class="timer__content">
                <h2 class="timer__title">{ "Timer" }</h2>
                <p class="timer__text">{ format_duration(duration, props.show_milliseconds) }</p>
            </div>
        </div>
    }
}

fn format_duration(duration: Duration, show_milliseconds: bool) -> String {
    let milliseconds = duration.num_milliseconds() % 1000;
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours();

    if show_milliseconds {
        if hours > 0 {
            format!(
                "{:02}:{:02}:{:02}:{:03}",
                hours, minutes, seconds, milliseconds
            )
        } else {
            format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
        }
    } else {
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
}
