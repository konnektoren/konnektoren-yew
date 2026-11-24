use crate::model::Tone;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PillProps {
    pub text: AttrValue,
    #[prop_or_default]
    pub tone: Tone,
    /// Before the text, decorative.
    #[prop_or_default]
    pub icon: Option<Html>,
    /// Accessible name, read as "{label}: {text}", e.g. "Result: 80%".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Tooltip.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Extra classes, e.g. an element class of the parent block.
    #[prop_or_default]
    pub class: Classes,
}

/// A small rounded label like "80%" or "You win", coloured by [`Tone`].
#[function_component(Pill)]
pub fn pill(props: &PillProps) -> Html {
    let aria_label = props
        .label
        .as_ref()
        .map(|label| AttrValue::from(format!("{label}: {}", props.text)));
    html! {
        <span class={classes!("pill", props.tone.modifier("pill"), props.class.clone())}
            aria-label={aria_label} title={props.title.clone()}>
            if let Some(icon) = &props.icon {
                <span class="pill__icon" aria-hidden="true">{icon.clone()}</span>
            }
            {props.text.clone()}
        </span>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn props(text: &'static str, tone: Tone) -> PillProps {
        PillProps {
            text: text.into(),
            tone,
            icon: None,
            label: None,
            title: None,
            class: Classes::new(),
        }
    }

    yew_preview::create_preview!(
        Pill,
        props("60%", Tone::Neutral),
        ("Good", props("90%", Tone::Good)),
        ("Info", props("72%", Tone::Info)),
        ("Warn", props("45%", Tone::Warn)),
        ("Bad", props("You lose", Tone::Bad)),
        (
            "With icon",
            PillProps {
                icon: Some(html! { "🏆" }),
                ..props("You win", Tone::Good)
            }
        ),
    );
}
