use crate::model::{DEFAULT_VOICE_NAME, SpeechConfig};
use gloo_timers::callback::Timeout;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{SpeechSynthesisUtterance, SpeechSynthesisVoice};

// The browser has one speech queue, even with multiple providers.
thread_local! {
    static ACTIVE: RefCell<Option<Rc<()>>> = const { RefCell::new(None) };
}

pub(super) fn speak(text: &str, lang: &str, config: &SpeechConfig) -> impl FnOnce() + use<> {
    let volume = config.volume.clamp(0.0, 1.0);
    let owner = Rc::new(());
    let scheduled = (volume.is_finite() && volume > 0.0 && !text.trim().is_empty())
        .then(web_sys::window)
        .flatten()
        .and_then(|window| window.speech_synthesis().ok())
        // Some WebViews expose the API as undefined.
        .filter(|speech| !speech.is_undefined() && !speech.is_null())
        .map(|speech| {
            let synth = speech.clone();
            let owner = owner.clone();
            let (text, lang, voice_uri) = (text.to_owned(), lang.to_owned(), config.voice.clone());
            let timer = Timeout::new(0, move || {
                let Ok(utterance) = SpeechSynthesisUtterance::new_with_text(&text) else {
                    return;
                };
                utterance.set_lang(&lang);
                utterance.set_volume(volume);
                if let Some(uri) = voice_uri.filter(|uri| uri != DEFAULT_VOICE_NAME) {
                    let voice = synth
                        .get_voices()
                        .iter()
                        .filter_map(|value| value.dyn_into::<SpeechSynthesisVoice>().ok())
                        .find(|voice| {
                            voice.voice_uri() == uri && language_matches(&voice.lang(), &lang)
                        });
                    if let Some(voice) = &voice {
                        utterance.set_voice(Some(voice));
                    }
                }
                ACTIVE.with(|active| active.replace(Some(owner)));
                synth.cancel();
                synth.speak(&utterance);
            });
            (speech, timer)
        });

    move || {
        if let Some((speech, timer)) = scheduled {
            drop(timer);
            let is_active = ACTIVE.with(|active| {
                let mut active = active.borrow_mut();
                if active
                    .as_ref()
                    .is_some_and(|current| Rc::ptr_eq(current, &owner))
                {
                    active.take();
                    true
                } else {
                    false
                }
            });
            if is_active {
                speech.cancel();
            }
        }
    }
}

/// Match primary language subtags so, for example, de-DE can speak de-AT.
fn language_matches(voice_language: &str, requested_language: &str) -> bool {
    let voice = voice_language.split('-').next().unwrap_or_default();
    let requested = requested_language.split('-').next().unwrap_or_default();
    !voice.is_empty() && voice.eq_ignore_ascii_case(requested)
}
