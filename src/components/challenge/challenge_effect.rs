use crate::components::ChallengePresenceComponent;
use crate::components::challenge::{ChallengeComponent, VibrateEffectComponent};
#[cfg(feature = "music")]
use crate::components::music::MusicComponent;
use crate::providers::{use_game_controller, use_selected_language};
use konnektoren_core::challenges::ChallengeVariant;
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::controller::GameControllerTrait;
use konnektoren_core::events::{ChallengeEvent, Event};
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeEffectProps {
    pub challenge: Challenge,
    #[prop_or_default]
    pub variant: Option<ChallengeVariant>,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
    #[prop_or(false)]
    pub preview: bool,
    /// Presence API. No presence is shown if `None`.
    #[prop_or_default]
    pub api_url: Option<AttrValue>,
    /// Sound played on a correct answer.
    #[prop_or_default]
    pub correct_sound_url: Option<AttrValue>,
    /// Sound played on a wrong answer.
    #[prop_or_default]
    pub incorrect_sound_url: Option<AttrValue>,
}

/// Wraps a [`ChallengeComponent`] with blink, sound and vibration feedback.
#[function_component(ChallengeEffectComponent)]
pub fn challenge_effect_component(props: &ChallengeEffectProps) -> Html {
    let game_controller = use_game_controller().controller.clone();
    let selected_language = use_selected_language();
    let effect = use_state(|| html! {});
    let counter = use_state(|| 0u32);

    {
        let game_controller = game_controller.clone();
        use_effect_with(props.challenge.clone(), move |challenge| {
            game_controller.publish_command(Command::Challenge(ChallengeCommand::Start(
                challenge.clone(),
            )));
            let _ = game_controller.save_game_state();
            || ()
        });
    }

    let handle_event = {
        let effect = effect.clone();
        let counter = counter.clone();
        let correct_sound_url = props.correct_sound_url.clone();
        let incorrect_sound_url = props.incorrect_sound_url.clone();
        Callback::from(move |event: Event| {
            let Event::Challenge(challenge_event) = event else {
                return;
            };
            let correct = match challenge_event {
                ChallengeEvent::SolvedCorrect(_) => true,
                ChallengeEvent::SolvedIncorrect(_) => false,
                ChallengeEvent::Error(e) => {
                    tracing::error!("Challenge error: {:?}", e);
                    return;
                }
                ChallengeEvent::Started | ChallengeEvent::Completed => return,
            };
            let c = *counter + 1;
            counter.set(c);
            let (modifier, sound_url) = if correct {
                ("correct", &correct_sound_url)
            } else {
                ("incorrect", &incorrect_sound_url)
            };
            #[cfg(feature = "music")]
            let sound = sound_url.as_ref().map(|url| {
                html! {
                    <MusicComponent
                        id={format!("challenge-effect__music--{modifier}-{c}")}
                        url={url.to_string()}
                        repeat={false}
                    />
                }
            });
            #[cfg(not(feature = "music"))]
            let sound = {
                let _ = sound_url;
                None::<Html>
            };
            effect.set(html! {
                <>
                    <div class={classes!("challenge-effect__blink", format!("challenge-effect__blink--{modifier}"))}></div>
                    {sound}
                    if !correct {
                        <VibrateEffectComponent duration={100} key={c} />
                    }
                </>
            });
            #[cfg(feature = "effects")]
            {
                let effect = effect.clone();
                gloo::timers::callback::Timeout::new(2000, move || effect.set(html! {})).forget();
            }
        })
    };

    let handle_command = {
        let on_finish = props.on_finish.clone();
        let game_controller = game_controller.clone();
        Callback::from(move |command: Command| {
            game_controller.publish_command(command.clone());
            if let (Some(on_finish), Command::Challenge(ChallengeCommand::Finish(result))) =
                (on_finish.as_ref(), command)
            {
                on_finish.emit(result.unwrap_or(ChallengeResult::Informative));
            }
        })
    };

    let presence = props.api_url.as_ref().map(|api_url| {
        html! {
            <ChallengePresenceComponent
                api_url={api_url.to_string()}
                challenge_id={props.challenge.challenge_config.id.clone()}
                read_only={props.preview}
            />
        }
    });

    html! {
        <div class="challenge-effect" id={format!("challenge-effect-{}", *counter)}>
            {(*effect).clone()}
            <div class="challenge-effect__content">
                {presence}
                <ChallengeComponent
                    challenge={props.challenge.clone()}
                    variant={props.variant.clone()}
                    on_event={handle_event}
                    on_command={handle_command}
                    language={selected_language.language.code()}
                    preview={props.preview}
                />
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::prelude::Game;
    use yew_preview::prelude::*;

    fn props(id: &str, preview: bool) -> ChallengeEffectProps {
        let challenge = Game::default().create_challenge(id).unwrap();
        ChallengeEffectProps {
            variant: challenge.challenge_config.variant.clone(),
            challenge,
            on_finish: None,
            preview,
            api_url: None,
            correct_sound_url: None,
            incorrect_sound_url: None,
        }
    }

    yew_preview::create_preview!(
        ChallengeEffectComponent,
        props("konnektoren-1", false),
        ("preview-mode", props("konnektoren-1", true)),
        ("Articles", props("articles-1", false)),
        ("Past tense", props("past-tense-1", false)),
        (
            "Sentence structure (ordering)",
            props("sentence-structure-1", false)
        ),
        (
            "Dialog observer",
            props("dialog-begruessung-observer", false)
        ),
        ("Dialog quiz", props("dialog-begruessung-quiz", false)),
    );
}
