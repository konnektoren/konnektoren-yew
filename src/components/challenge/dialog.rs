//! Dialog challenge component — renders a two-speaker scripted conversation in
//! either **Observer** (watch) or **Quiz** (pick the correct line) mode.
//!
//! Sub-components:
//! - [`DialogSpeakerBadge`]  — speaker avatar shown in the top bar
//! - [`DialogBubble`]        — single DaisyUI chat bubble (left / right)
//! - [`DialogOptions`]       — multiple-choice buttons for a quiz turn
//! - [`DialogComponent`]     — main orchestrator

use crate::i18n::use_i18n;
use konnektoren_core::challenges::{
    ChallengeInput, ChallengeResult, Dialog, DialogAnswer, Speaker,
};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::{ChallengeEvent, Event};
use yew::prelude::*;

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Render a speaker's `icon` field as either an `<i>` (FontAwesome / CSS
/// class, detected by `"fa-"` prefix) or an `<img>` (URL / data URI).
/// Falls back to a 👤 emoji when `None`.
///
/// Matches the convention used across the codebase (see `vocabulary.rs`,
/// `placeholder.rs`, `achievement.rs`).
fn render_speaker_icon(icon: Option<&str>) -> Html {
    match icon {
        None => html! { <span>{"👤"}</span> },
        Some(cls) if cls.starts_with("fa-") => html! { <i class={cls.to_string()}></i> },
        Some(src) => html! {
            <img src={src.to_string()} class="dialog__avatar-img" alt="speaker avatar" />
        },
    }
}

/// Render the dialog's `image` field as a full-width scene header.
///
/// * FontAwesome / CSS class (detected by `"fa-"` prefix) → large centred
///   icon in a tinted band
/// * Anything else → `<img>` banner (URL, data URI, asset path, …)
fn render_scene(image: &str) -> Html {
    if image.starts_with("fa-") {
        html! {
            <div class="dialog__scene dialog__scene--icon">
                <i class={image.to_string()}></i>
            </div>
        }
    } else {
        html! {
            <div class="dialog__scene">
                <img src={image.to_string()} class="dialog__scene-image" alt="scene" />
            </div>
        }
    }
}

// ─── DialogSpeakerBadge ───────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct DialogSpeakerBadgeProps {
    pub speaker: Speaker,
    #[prop_or(false)]
    pub is_active: bool,
}

#[function_component(DialogSpeakerBadge)]
pub fn dialog_speaker_badge(props: &DialogSpeakerBadgeProps) -> Html {
    html! {
        <div class={classes!(
            "dialog__speaker-badge",
            props.is_active.then_some("dialog__speaker-badge--active")
        )}>
            <div class="dialog__speaker-avatar">
                { render_speaker_icon(props.speaker.icon.as_deref()) }
            </div>
            <span class="dialog__speaker-name">{ &props.speaker.name }</span>
        </div>
    }
}

// ─── DialogBubble ─────────────────────────────────────────────────────────────

/// Feedback state of an answered quiz bubble.
#[derive(Clone, PartialEq)]
pub enum BubbleState {
    Correct,
    Incorrect,
}

#[derive(Properties, PartialEq)]
pub struct DialogBubbleProps {
    pub speaker: Speaker,
    pub text: String,
    /// `true`  → right side (`chat-end`), `false` → left side (`chat-start`).
    /// Determined by the speaker's position in [`Dialog::speakers`], not by
    /// DOM order — so the alignment is stable as the chat log grows.
    pub is_end: bool,
    #[prop_or_default]
    pub state: Option<BubbleState>,
}

#[function_component(DialogBubble)]
pub fn dialog_bubble(props: &DialogBubbleProps) -> Html {
    let side = if props.is_end {
        "chat-end"
    } else {
        "chat-start"
    };

    // Colour: feedback overrides the default speaker-position colour.
    let bubble_colour = match &props.state {
        Some(BubbleState::Correct) => "chat-bubble-success",
        Some(BubbleState::Incorrect) => "chat-bubble-error",
        None if props.is_end => "chat-bubble-primary",
        None => "chat-bubble-secondary",
    };

    html! {
        <div class={classes!("chat", side, "dialog__turn")}>
            <div class="chat-image avatar">
                <div class="dialog__avatar-circle">
                    { render_speaker_icon(props.speaker.icon.as_deref()) }
                </div>
            </div>
            <div class="chat-header dialog__turn-header">{ &props.speaker.name }</div>
            <div class={classes!("chat-bubble", "dialog__bubble", bubble_colour)}>
                { &props.text }
            </div>
        </div>
    }
}

// ─── DialogOptions ────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct DialogOptionsProps {
    pub options: Vec<String>,
    #[prop_or_default]
    pub selected: Option<usize>,
    /// Index of the correct option — only revealed after the player commits.
    #[prop_or_default]
    pub correct: Option<usize>,
    pub on_select: Callback<usize>,
}

#[function_component(DialogOptions)]
pub fn dialog_options(props: &DialogOptionsProps) -> Html {
    let already_answered = props.selected.is_some();

    html! {
        <div class="dialog__options">
            { props.options.iter().enumerate().map(|(i, option)| {
                let is_selected      = props.selected == Some(i);
                let is_correct       = props.correct  == Some(i);
                let is_wrong         = is_selected && already_answered && !is_correct;
                let is_right_pick    = is_selected && is_correct;

                let class = classes!(
                    "dialog__option",
                    is_correct.then_some("dialog__option--correct"),
                    is_wrong.then_some("dialog__option--incorrect"),
                    is_right_pick.then_some("dialog__option--selected-correct"),
                );

                let on_click = {
                    let cb = props.on_select.clone();
                    Callback::from(move |_: MouseEvent| cb.emit(i))
                };

                html! {
                    <button class={class} onclick={on_click} disabled={already_answered}>
                        { option }
                    </button>
                }
            }).collect::<Html>() }
        </div>
    }
}

// ─── DialogComponent ──────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct DialogComponentProps {
    pub challenge: Dialog,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
    /// `false` = Observer (watch only), `true` = Quiz (pick the correct line).
    #[prop_or(false)]
    pub quiz_mode: bool,
}

#[function_component(DialogComponent)]
pub fn dialog_component(props: &DialogComponentProps) -> Html {
    let i18n = use_i18n();

    // ── state ──────────────────────────────────────────────────────────────
    let current_turn = use_state(|| 0_usize);
    let challenge_result = use_state(|| ChallengeResult::Dialog(Vec::new()));
    let selected_option = use_state(|| None::<usize>);
    let is_answered = use_state(|| false);

    let dialog = &props.challenge;
    let total_turns = dialog.turns.len();

    // Guard: nothing to show.
    if total_turns == 0 || *current_turn >= total_turns {
        return html! {};
    }

    let current_turn_data = &dialog.turns[*current_turn];
    let is_quiz_turn = props.quiz_mode && current_turn_data.is_quiz_turn();
    let show_options = is_quiz_turn && !*is_answered;

    // ── callbacks ──────────────────────────────────────────────────────────

    let handle_option_select = {
        let turn_idx = *current_turn;
        let selected_option = selected_option.clone();
        let is_answered = is_answered.clone();
        let challenge_result = challenge_result.clone();
        let on_event = props.on_event.clone();
        let challenge = dialog.clone();

        Callback::from(move |option_index: usize| {
            if *is_answered {
                return;
            }
            selected_option.set(Some(option_index));
            is_answered.set(true);

            // Record the answer in the challenge result.
            let answer = DialogAnswer {
                turn_index: turn_idx,
                selected_option: option_index,
            };
            let mut result = (*challenge_result).clone();
            let _ = result.add_input(ChallengeInput::Dialog(answer));
            challenge_result.set(result);

            // Emit the correctness event.
            if let Some(turn) = challenge.turns.get(turn_idx) {
                if let Some(correct) = turn.correct_option {
                    if let Some(on_event) = &on_event {
                        if option_index == correct {
                            on_event
                                .emit(Event::Challenge(ChallengeEvent::SolvedCorrect(turn_idx)));
                        } else {
                            on_event
                                .emit(Event::Challenge(ChallengeEvent::SolvedIncorrect(turn_idx)));
                        }
                    }
                }
            }
        })
    };

    let handle_next = {
        let current_turn = current_turn.clone();
        let selected_option = selected_option.clone();
        let is_answered = is_answered.clone();
        let challenge_result = challenge_result.clone();
        let on_command = props.on_command.clone();
        let total = total_turns;

        Callback::from(move |_: MouseEvent| {
            let next = *current_turn + 1;
            if next >= total {
                if let Some(cmd) = &on_command {
                    cmd.emit(Command::Challenge(ChallengeCommand::Finish(Some(
                        (*challenge_result).clone(),
                    ))));
                }
            } else {
                current_turn.set(next);
                selected_option.set(None);
                is_answered.set(false);
                if let Some(cmd) = &on_command {
                    cmd.emit(Command::Challenge(ChallengeCommand::NextTask));
                }
            }
        })
    };

    // ── helpers ────────────────────────────────────────────────────────────

    /// Returns `true` when `speaker_id` belongs to the second speaker slot,
    /// meaning the bubble should appear on the right (`chat-end`).
    let is_end_speaker = |speaker_id: &str| -> bool { dialog.speakers[1].id == speaker_id };

    // ── past turns ─────────────────────────────────────────────────────────
    //
    // IMPORTANT: we use `DialogBubble` — NOT `MessageComp` from yew-chat —
    // for all past turns.  `MessageComp` relies on CSS `nth-child(odd/even)`
    // for left/right alignment, which re-evaluates as the list grows and
    // causes bubbles to jump sides on every "Next" click.  `DialogBubble`
    // derives alignment from the speaker's fixed identity, so it is stable.

    let past_bubbles = (0..*current_turn)
        .map(|i| {
            let turn = &dialog.turns[i];
            let speaker = dialog
                .speaker_by_id(&turn.speaker)
                .cloned()
                .unwrap_or_default();
            let is_end = is_end_speaker(&speaker.id);
            html! {
                <DialogBubble
                    key={i}
                    speaker={speaker}
                    text={turn.text.clone()}
                    is_end={is_end}
                />
            }
        })
        .collect::<Html>();

    // ── active turn rendering ──────────────────────────────────────────────

    let active_speaker = dialog
        .speaker_by_id(&current_turn_data.speaker)
        .cloned()
        .unwrap_or_default();
    let current_is_end = is_end_speaker(&active_speaker.id);

    let current_content = if show_options {
        // Quiz turn awaiting input — hide the text, show options instead.
        let options = current_turn_data.options.clone().unwrap_or_default();

        // Show a "ghost" bubble on the correct side so the player knows who
        // is speaking, but keep the text hidden until they answer.
        html! {
            <>
                <div class={classes!(
                    "chat",
                    if current_is_end { "chat-end" } else { "chat-start" },
                    "dialog__turn",
                )}>
                    <div class="chat-image avatar">
                        <div class="dialog__avatar-circle">
                            { render_speaker_icon(active_speaker.icon.as_deref()) }
                        </div>
                    </div>
                    <div class="chat-header dialog__turn-header">{ &active_speaker.name }</div>
                    <div class="chat-bubble dialog__bubble dialog__bubble--pending">{"…"}</div>
                </div>

                <div class="dialog__quiz-prompt">
                    <p class="dialog__quiz-prompt-text">
                        { i18n.t("dialog.choose_response") }
                    </p>
                    <DialogOptions options={options} on_select={handle_option_select} />
                </div>
            </>
        }
    } else {
        // Observer turn OR quiz turn after answering — show bubble + Next/Finish.
        let bubble_state = if *is_answered {
            current_turn_data.correct_option.map(|correct| {
                if *selected_option == Some(correct) {
                    BubbleState::Correct
                } else {
                    BubbleState::Incorrect
                }
            })
        } else {
            None
        };

        // On a wrong pick, reveal the correct line below the bubble.
        let correct_hint = if matches!(&bubble_state, Some(BubbleState::Incorrect)) {
            let correct_text = current_turn_data
                .correct_option
                .and_then(|ci| {
                    current_turn_data
                        .options
                        .as_ref()
                        .and_then(|opts| opts.get(ci))
                })
                .cloned()
                .unwrap_or_default();
            html! {
                <div class="dialog__correct-hint">
                    <span class="dialog__correct-hint-label">
                        { i18n.t("Correct answer: ") }
                    </span>
                    { correct_text }
                </div>
            }
        } else {
            html! {}
        };

        let is_last = *current_turn + 1 >= total_turns;
        let btn_text = if is_last {
            i18n.t("Finish")
        } else {
            i18n.t("Next")
        };

        html! {
            <>
                <DialogBubble
                    speaker={active_speaker.clone()}
                    text={current_turn_data.text.clone()}
                    is_end={current_is_end}
                    state={bubble_state}
                />
                { correct_hint }
                <div class="dialog__next">
                    <button class="dialog__next-btn btn btn-primary" onclick={handle_next}>
                        { btn_text }
                    </button>
                </div>
            </>
        }
    };

    // ── full layout ────────────────────────────────────────────────────────
    html! {
        <div class="dialog">
            if let Some(image) = &dialog.image {
                { render_scene(image) }
            }

            if let Some(scenario) = &dialog.scenario {
                <div class="dialog__scenario">{ scenario }</div>
            }

            <div class="dialog__speakers-bar">
                { dialog.speakers.iter().map(|s| {
                    let is_active = s.id == current_turn_data.speaker;
                    html! {
                        <DialogSpeakerBadge
                            key={s.id.clone()}
                            speaker={s.clone()}
                            is_active={is_active}
                        />
                    }
                }).collect::<Html>() }
            </div>

            <div class="dialog__chat">
                { past_bubbles }
                { current_content }
            </div>
        </div>
    }
}

// ─── yew-preview ─────────────────────────────────────────────────────────────

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{DialogTurn, Speaker};
    use yew_preview::prelude::*;

    // ── image avatars (self-contained SVG data URIs — no network needed) ───

    /// Simple coloured circle avatar for Anna (rose).
    const ANNA_IMG: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' \
        viewBox='0 0 100 100'%3E\
        %3Ccircle cx='50' cy='50' r='50' fill='%23f43f5e'/%3E\
        %3Ccircle cx='50' cy='38' r='18' fill='%23fce7f3'/%3E\
        %3Cellipse cx='50' cy='90' rx='28' ry='20' fill='%23fce7f3'/%3E\
        %3C/svg%3E";

    /// Simple coloured circle avatar for Ben (blue).
    const BEN_IMG: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' \
        viewBox='0 0 100 100'%3E\
        %3Ccircle cx='50' cy='50' r='50' fill='%233b82f6'/%3E\
        %3Ccircle cx='50' cy='38' r='18' fill='%23dbeafe'/%3E\
        %3Cellipse cx='50' cy='90' rx='28' ry='20' fill='%23dbeafe'/%3E\
        %3C/svg%3E";

    fn make_image_dialog() -> Dialog {
        Dialog {
            id: "dialog_image_preview".to_string(),
            name: "Begrüßung am Morgen 👋 (with avatars)".to_string(),
            description: "Same dialog, but speakers have image avatars.".to_string(),
            lang: "de".to_string(),
            scenario: Some("Two neighbours meet on a Monday morning.".to_string()),
            image: Some("https://picsum.photos/600/200".to_string()),
            speakers: [
                Speaker {
                    id: "anna".to_string(),
                    name: "Anna".to_string(),
                    icon: Some(ANNA_IMG.to_string()),
                },
                Speaker {
                    id: "ben".to_string(),
                    name: "Ben".to_string(),
                    icon: Some(BEN_IMG.to_string()),
                },
            ],
            turns: vec![
                DialogTurn {
                    speaker: "anna".to_string(),
                    text: "Guten Morgen, Ben!".to_string(),
                    audio: None,
                    options: None,
                    correct_option: None,
                },
                DialogTurn {
                    speaker: "ben".to_string(),
                    text: "Guten Morgen, Anna! Wie geht's?".to_string(),
                    audio: None,
                    options: Some(vec![
                        "Guten Morgen, Anna! Wie geht's?".to_string(),
                        "Gute Nacht, Anna!".to_string(),
                        "Auf Wiedersehen, Anna!".to_string(),
                    ]),
                    correct_option: Some(0),
                },
                DialogTurn {
                    speaker: "anna".to_string(),
                    text: "Gut, danke! Und dir?".to_string(),
                    audio: None,
                    options: None,
                    correct_option: None,
                },
                DialogTurn {
                    speaker: "ben".to_string(),
                    text: "Auch gut, danke!".to_string(),
                    audio: None,
                    options: Some(vec![
                        "Schlecht, danke.".to_string(),
                        "Ich weiß nicht.".to_string(),
                        "Auch gut, danke!".to_string(),
                    ]),
                    correct_option: Some(2),
                },
                DialogTurn {
                    speaker: "anna".to_string(),
                    text: "Schön! Tschüss, Ben!".to_string(),
                    audio: None,
                    options: None,
                    correct_option: None,
                },
                DialogTurn {
                    speaker: "ben".to_string(),
                    text: "Tschüss! Einen schönen Tag noch!".to_string(),
                    audio: None,
                    options: Some(vec![
                        "Tschüss! Einen schönen Tag noch!".to_string(),
                        "Guten Morgen!".to_string(),
                        "Wie geht's?".to_string(),
                    ]),
                    correct_option: Some(0),
                },
            ],
        }
    }

    yew_preview::create_preview!(
        DialogComponent,
        // Default: Observer mode with FontAwesome icons (from Dialog::default())
        DialogComponentProps {
            challenge: Dialog::default(),
            on_command: None,
            on_event: None,
            quiz_mode: false,
        },
        (
            "Quiz mode (icons)",
            DialogComponentProps {
                challenge: Dialog::default(),
                on_command: None,
                on_event: None,
                quiz_mode: true,
            }
        ),
        (
            "Observer mode (images)",
            DialogComponentProps {
                challenge: make_image_dialog(),
                on_command: None,
                on_event: None,
                quiz_mode: false,
            }
        ),
        (
            "Quiz mode (images)",
            DialogComponentProps {
                challenge: make_image_dialog(),
                on_command: None,
                on_event: None,
                quiz_mode: true,
            }
        ),
    );
}
