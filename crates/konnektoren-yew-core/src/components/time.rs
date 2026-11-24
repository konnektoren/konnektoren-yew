use crate::model::{TimeValue, display_timestamp, format_time};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimeProps {
    pub value: TimeValue,
    #[prop_or_default]
    pub class: Classes,
}

/// A `<time>` element: `2026-11-22 14:30 UTC`, or a stored timestamp as
/// `2026-11-22 14:30`.
#[function_component(Time)]
pub fn time(props: &TimeProps) -> Html {
    let (datetime, text): (AttrValue, String) = match &props.value {
        TimeValue::At(time) => (time.to_rfc3339().into(), format_time(*time)),
        TimeValue::Raw(timestamp) => (timestamp.clone(), display_timestamp(timestamp)),
    };
    html! { <time class={props.class.clone()} {datetime}>{text}</time> }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use chrono::{TimeZone, Utc};
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        Time,
        TimeProps {
            value: Utc
                .with_ymd_and_hms(2026, 11, 22, 14, 30, 0)
                .single()
                .map_or(TimeValue::Raw("".into()), TimeValue::At),
            class: Classes::new(),
        },
        (
            "Stored timestamp",
            TimeProps {
                value: TimeValue::Raw("2026-11-21T09:05:00.000Z".into()),
                class: Classes::new(),
            }
        ),
    );
}
