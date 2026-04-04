//! Completion screen shown after finishing a **Dialog Observer** challenge.
//!
//! Observer mode has no interactive turns, so there is nothing to grade.
//! This component celebrates the player finishing the conversation and shows
//! scene context (image / icon, scenario text, speakers) instead of a table.

use crate::i18n::use_i18n;
use konnektoren_core::challenges::Dialog;
use yew::prelude::*;

// ─── helpers (same convention as the rest of the codebase) ───────────────────

/// `"fa-*"` → `<i>` element, anything else → `<img>`.
fn render_speaker_icon(icon: Option<&str>) -> Html {
    match icon {
        None => html! { <span>{"👤"}</span> },
        Some(cls) if cls.starts_with("fa-") => html! { <i class={cls.to_string()}></i> },
        Some(src) => html! {
            <img src={src.to_string()}
                 class="dialog-observer-result__avatar-img"
                 alt="speaker avatar" />
        },
    }
}

/// Render the `Dialog.image` field as a scene banner at the top of the card.
fn render_scene(image: &str) -> Html {
    if image.starts_with("fa-") {
        html! {
            <div class="dialog-observer-result__scene dialog-observer-result__scene--icon">
                <i class={image.to_string()}></i>
            </div>
        }
    } else {
        html! {
            <div class="dialog-observer-result__scene">
                <img src={image.to_string()}
                     class="dialog-observer-result__scene-image"
                     alt="scene" />
            </div>
        }
    }
}

// ─── component ────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct DialogObserverResultComponentProps {
    pub challenge: Dialog,
}

#[function_component(DialogObserverResultComponent)]
pub fn dialog_observer_result_component(props: &DialogObserverResultComponentProps) -> Html {
    let i18n = use_i18n();
    let dialog = &props.challenge;

    html! {
        <div class="dialog-observer-result">
            // ── scene image / icon ────────────────────────────────────────
            if let Some(image) = &dialog.image {
                { render_scene(image) }
            }

            // ── completion badge ──────────────────────────────────────────
            <div class="dialog-observer-result__check">
                <i class="fa-solid fa-circle-check"></i>
            </div>

            // ── dialog title ──────────────────────────────────────────────
            <h2 class="dialog-observer-result__title">{ &dialog.name }</h2>

            // ── scenario ──────────────────────────────────────────────────
            if let Some(scenario) = &dialog.scenario {
                <p class="dialog-observer-result__scenario">{ scenario }</p>
            }

            // ── speaker cards ─────────────────────────────────────────────
            <div class="dialog-observer-result__speakers">
                { for dialog.speakers.iter().map(|s| html! {
                    <div class="dialog-observer-result__speaker">
                        <div class="dialog-observer-result__speaker-avatar">
                            { render_speaker_icon(s.icon.as_deref()) }
                        </div>
                        <span class="dialog-observer-result__speaker-name">
                            { &s.name }
                        </span>
                    </div>
                }) }
            </div>

            // ── completion message ─────────────────────────────────────────
            <p class="dialog-observer-result__message">
                { i18n.t("dialog.observer_complete") }
            </p>
        </div>
    }
}

// ─── yew-preview ─────────────────────────────────────────────────────────────

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::{DialogTurn, Speaker};
    use yew_preview::prelude::*;

    fn make_with_scene_image() -> Dialog {
        Dialog {
            id: "dialog_observer_preview".to_string(),
            name: "Beim Bäcker 🥐".to_string(),
            description: "A customer buys bread at a bakery.".to_string(),
            lang: "de".to_string(),
            scenario: Some("At a bakery in Berlin on a Tuesday morning.".to_string()),
            image: Some("https://picsum.photos/600/200".to_string()),
            speakers: [
                Speaker {
                    id: "kunde".to_string(),
                    name: "Kunde".to_string(),
                    icon: Some("fa-solid fa-user".to_string()),
                },
                Speaker {
                    id: "baecker".to_string(),
                    name: "Bäcker".to_string(),
                    icon: Some("fa-solid fa-bread-slice".to_string()),
                },
            ],
            turns: vec![
                DialogTurn {
                    speaker: "kunde".to_string(),
                    text: "Guten Morgen! Vier Brötchen, bitte.".to_string(),
                    audio: None,
                    options: None,
                    correct_option: None,
                },
                DialogTurn {
                    speaker: "baecker".to_string(),
                    text: "Guten Morgen! Das macht 1,20 Euro.".to_string(),
                    audio: None,
                    options: None,
                    correct_option: None,
                },
            ],
        }
    }

    yew_preview::create_preview!(
        DialogObserverResultComponent,
        // Default: FontAwesome scene icon from Dialog::default()
        DialogObserverResultComponentProps {
            challenge: Dialog::default(),
        },
        (
            "With scene image",
            DialogObserverResultComponentProps {
                challenge: make_with_scene_image(),
            }
        ),
    );
}
