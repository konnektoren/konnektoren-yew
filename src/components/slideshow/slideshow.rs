use super::slide::Slide;
use yew::prelude::*;

/// Content displayed by one step of a [`Slideshow`]. All text is supplied by the caller so the
/// component can be used with each application's translation provider.
#[derive(Clone, PartialEq)]
pub struct SlideshowStep {
    pub icon: AttrValue,
    pub title: AttrValue,
    pub description: AttrValue,
    pub content: Html,
}

/// Labels and content for a [`Slideshow`].
#[derive(Clone, PartialEq)]
pub struct SlideshowConfig {
    pub steps: Vec<SlideshowStep>,
    pub aria_label: AttrValue,
    pub previous_label: AttrValue,
    pub next_label: AttrValue,
    pub finish_label: AttrValue,
    pub skip_label: Option<AttrValue>,
}

#[derive(Properties, PartialEq)]
pub struct SlideshowProps {
    pub config: SlideshowConfig,
    pub on_finish: Callback<()>,
    #[prop_or_default]
    pub on_skip: Option<Callback<()>>,
}

/// A configurable step-by-step introduction with progress dots and navigation controls.
#[function_component(Slideshow)]
pub fn slideshow(props: &SlideshowProps) -> Html {
    let current = use_state(|| 0usize);
    let total = props.config.steps.len();
    if props.config.steps.get(*current).is_none() {
        return Html::default();
    }
    let is_last = *current + 1 == total;

    let on_next = {
        let current = current.clone();
        let on_finish = props.on_finish.clone();
        Callback::from(move |_: MouseEvent| {
            if *current + 1 == total {
                on_finish.emit(());
            } else {
                current.set(*current + 1);
            }
        })
    };
    let on_previous = {
        let current = current.clone();
        Callback::from(move |_: MouseEvent| current.set(current.saturating_sub(1)))
    };
    let skip = match (&props.config.skip_label, &props.on_skip) {
        (Some(label), Some(on_skip)) => {
            let on_skip = on_skip.clone();
            html! {
                <button type="button" class="slideshow__skip" onclick={Callback::from(move |_: MouseEvent| on_skip.emit(()))}>
                    {label}
                </button>
            }
        }
        _ => Html::default(),
    };

    html! {
        <section class="slideshow" aria-roledescription="carousel" aria-label={props.config.aria_label.clone()}>
            {skip}
            <div class="slideshow__slides">
                {for props.config.steps.iter().cloned().enumerate().map(|(index, step)| html! {
                    <Slide key={index} {step} active={index == *current} />
                })}
            </div>
            <div class="slideshow__dots" role="status" aria-live="polite" aria-label={format!("{} / {}", *current + 1, total)}>
                {for (0..total).map(|index| html! {
                    <span key={index} class={classes!("slideshow__dot", (index == *current).then_some("slideshow__dot--active"))}></span>
                })}
            </div>
            <div class="slideshow__controls">
                <button type="button" class="slideshow__button slideshow__button--previous btn btn-ghost" onclick={on_previous} disabled={*current == 0}>
                    {props.config.previous_label.clone()}
                </button>
                <button type="button" class="slideshow__button slideshow__button--next btn btn-primary" onclick={on_next}>
                    {if is_last { props.config.finish_label.clone() } else { props.config.next_label.clone() }}
                </button>
            </div>
        </section>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn config(steps: Vec<SlideshowStep>) -> SlideshowConfig {
        SlideshowConfig {
            steps,
            aria_label: "Welcome tour".into(),
            previous_label: "Previous".into(),
            next_label: "Next".into(),
            finish_label: "Start".into(),
            skip_label: Some("Skip".into()),
        }
    }

    fn steps() -> Vec<SlideshowStep> {
        vec![
            SlideshowStep {
                icon: "🎹".into(),
                title: "Welcome".into(),
                description: "Connect a keyboard and start with the first piece.".into(),
                content: html! { <p>{"Any MIDI keyboard works."}</p> },
            },
            SlideshowStep {
                icon: "🎵".into(),
                title: "Read the notes".into(),
                description: "Notes fall towards the keys, one lane per pitch.".into(),
                content: html! { <p>{"Wait for the note to reach the line."}</p> },
            },
        ]
    }

    yew_preview::create_preview!(
        Slideshow,
        SlideshowProps {
            config: config(steps()),
            on_finish: Callback::from(|_| {}),
            on_skip: Some(Callback::from(|_| {})),
        },
        (
            "three slides",
            SlideshowProps {
                config: {
                    let mut steps = steps();
                    steps.push(SlideshowStep {
                        icon: "✨".into(),
                        title: "Start playing".into(),
                        description: "You are ready for your first exercise.".into(),
                        content: html! { <p>{"Choose a challenge to begin."}</p> },
                    });
                    config(steps)
                },
                on_finish: Callback::from(|_| {}),
                on_skip: Some(Callback::from(|_| {})),
            }
        ),
        (
            "language selection",
            SlideshowProps {
                config: config(vec![
                    SlideshowStep {
                        icon: "🌐".into(),
                        title: "Choose your language".into(),
                        description: "Select the language you want to learn in.".into(),
                        content: html! { <crate::components::SelectLanguage /> },
                    },
                    SlideshowStep {
                        icon: "🎯".into(),
                        title: "Choose your level".into(),
                        description: "You can change both settings any time.".into(),
                        content: html! {
                            <crate::components::SelectLevelComp
                                current={0}
                                levels={Vec::new()}
                                on_select={Callback::noop()}
                            />
                        },
                    },
                ]),
                on_finish: Callback::from(|_| {}),
                on_skip: Some(Callback::from(|_| {})),
            }
        ),
        (
            "dark mode selection",
            SlideshowProps {
                config: config(vec![
                    SlideshowStep {
                        icon: "🌙".into(),
                        title: "Choose dark mode".into(),
                        description: "Switch between light and dark mode whenever you like.".into(),
                        content: html! {
                            <crate::components::SelectTheme
                                themes={Some(vec![crate::model::Theme::Light, crate::model::Theme::Dark])}
                            />
                        },
                    },
                    SlideshowStep {
                        icon: "✨".into(),
                        title: "Make it yours".into(),
                        description: "Your theme choice is applied across the app.".into(),
                        content: Html::default(),
                    },
                ]),
                on_finish: Callback::from(|_| {}),
                on_skip: Some(Callback::from(|_| {})),
            }
        ),
    );
}
