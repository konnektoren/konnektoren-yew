use super::SlideshowStep;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SlideProps {
    pub step: SlideshowStep,
    pub active: bool,
}

#[function_component(Slide)]
pub fn slide(props: &SlideProps) -> Html {
    html! {
        <div
            class={classes!("slideshow__slide", props.active.then_some("slideshow__slide--active"))}
            aria-hidden={(!props.active).to_string()}
        >
            <div class="slideshow__step-icon" aria-hidden="true">{props.step.icon.clone()}</div>
            <h2 class="slideshow__title">{props.step.title.clone()}</h2>
            <p class="slideshow__description">{props.step.description.clone()}</p>
            <div class="slideshow__content">{props.step.content.clone()}</div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        Slide,
        SlideProps {
            step: SlideshowStep {
                icon: "🎹".into(),
                title: "Welcome".into(),
                description: "Connect a keyboard and start with the first piece.".into(),
                content: html! { <p>{"Any MIDI keyboard works."}</p> },
            },
            active: true,
        },
        (
            "language selection",
            SlideProps {
                step: SlideshowStep {
                    icon: "🌐".into(),
                    title: "Choose your language".into(),
                    description: "Select the language you want to learn in.".into(),
                    content: html! { <crate::components::SelectLanguage /> },
                },
                active: true,
            }
        ),
        (
            "dark mode selection",
            SlideProps {
                step: SlideshowStep {
                    icon: "🌙".into(),
                    title: "Choose dark mode".into(),
                    description: "Switch between light and dark mode whenever you like.".into(),
                    content: html! {
                        <crate::components::SelectTheme
                            themes={Some(vec![crate::model::Theme::Light, crate::model::Theme::Dark])}
                        />
                    },
                },
                active: true,
            }
        ),
    );
}
