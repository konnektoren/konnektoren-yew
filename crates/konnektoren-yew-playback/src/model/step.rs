use serde::de::{DeserializeOwned, Error as _};
use serde::{Deserialize, Deserializer};

/// Step data for hosts without custom step keys. Rejects every unknown key.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NoExtension {}

/// What playback does with a step's target once the step is due.
#[derive(Clone, Debug, PartialEq)]
pub enum TargetAction {
    /// Click the target (`click: true`).
    Click,
    /// Show the click effect without clicking (`tap: true`), e.g. on a link
    /// that would leave the excursion.
    Tap,
    /// Set an input's value and dispatch an `input` event (`fill: "text"`).
    Fill(String),
    /// Press and release a key on the target (`press: Enter`).
    Press(String),
}

/// Placement of a fixed narration message within the viewport.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// One scene of an excursion: a message, an optional target and what happens to it.
///
/// `X` holds the host application's own keys, such as MIDI notes in a music
/// app. Its fields share the step's YAML mapping; derive `Deserialize` with
/// `deny_unknown_fields` so typos are still rejected:
///
/// ```yaml
/// - say: "Play the first notes."
///   page: practice
///   selector: "#piano"
///   highlight: true
///   midi: [{ pitch: 60, duration_ms: 300, gap_ms: 100 }]   # host key
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Step<X = NoExtension> {
    /// Message shown while the step runs, translated by the host.
    pub say: String,
    /// Speak the translated message while this step is displayed.
    pub speak: bool,
    /// Optional position of the fixed notification.
    pub position: Option<NotificationPosition>,
    /// Replace this step with another excursion's steps. Exclusive with all other keys.
    pub include: Option<String>,
    /// Host-defined view, kept for following steps until changed. See [`crate::Excursion::page_at`].
    pub page: Option<String>,
    /// CSS selector of the step's target.
    pub selector: Option<String>,
    /// Frame the target while the step is shown.
    pub highlight: bool,
    /// Keep scrolling a moving target into view.
    pub follow: bool,
    /// Skip the action instead of waiting when the target is missing.
    pub optional: bool,
    pub action: Option<TargetAction>,
    /// Fixed time before the step's action. Without it the runtime's default
    /// delay applies, scaled by playback speed.
    pub delay_ms: Option<u32>,
    /// Host-specific data.
    pub extension: X,
}

impl<X: Default> Default for Step<X> {
    fn default() -> Self {
        Self {
            say: String::new(),
            speak: false,
            position: None,
            include: None,
            page: None,
            selector: None,
            highlight: false,
            follow: false,
            optional: false,
            action: None,
            delay_ms: None,
            extension: X::default(),
        }
    }
}

impl<X: Default> Step<X> {
    pub fn say(message: impl Into<String>) -> Self {
        Self {
            say: message.into(),
            ..Self::default()
        }
    }

    pub fn include(id: impl Into<String>) -> Self {
        Self {
            include: Some(id.into()),
            ..Self::default()
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CoreStep {
    #[serde(default)]
    say: String,
    #[serde(default)]
    speak: bool,
    position: Option<NotificationPosition>,
    include: Option<String>,
    page: Option<String>,
    selector: Option<String>,
    #[serde(default)]
    highlight: bool,
    #[serde(default)]
    follow: bool,
    #[serde(default)]
    optional: bool,
    #[serde(default)]
    click: bool,
    #[serde(default)]
    tap: bool,
    fill: Option<String>,
    press: Option<String>,
    delay_ms: Option<u32>,
}

const CORE_KEYS: &[&str] = &[
    "say",
    "speak",
    "position",
    "include",
    "page",
    "selector",
    "highlight",
    "follow",
    "optional",
    "click",
    "tap",
    "fill",
    "press",
    "delay_ms",
];

impl<'de, X: DeserializeOwned> Deserialize<'de> for Step<X> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut entries = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let has_extension = entries.keys().any(|key| !CORE_KEYS.contains(&key.as_str()));
        let mut core = serde_json::Map::new();
        for key in CORE_KEYS {
            if let Some(value) = entries.remove(*key) {
                core.insert((*key).to_owned(), value);
            }
        }
        if core.contains_key("include") && (core.len() > 1 || has_extension) {
            return Err(D::Error::custom(
                "an include step cannot contain other instructions",
            ));
        }
        let core: CoreStep = serde_json::from_value(core.into()).map_err(D::Error::custom)?;
        let extension: X = serde_json::from_value(entries.into()).map_err(D::Error::custom)?;
        let actions = [
            core.click.then_some(TargetAction::Click),
            core.tap.then_some(TargetAction::Tap),
            core.fill.map(TargetAction::Fill),
            core.press.map(TargetAction::Press),
        ];
        let mut actions = actions.into_iter().flatten();
        let action = actions.next();
        if actions.next().is_some() {
            return Err(D::Error::custom(
                "a step can only click, tap, fill or press, not several",
            ));
        }
        if action.is_some() && core.selector.is_none() {
            return Err(D::Error::custom("a target action needs a selector"));
        }
        Ok(Self {
            say: core.say,
            speak: core.speak,
            position: core.position,
            include: core.include,
            page: core.page,
            selector: core.selector,
            highlight: core.highlight,
            follow: core.follow,
            optional: core.optional,
            action,
            delay_ms: core.delay_ms,
            extension,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse<X: DeserializeOwned>(yaml: &str) -> Result<Step<X>, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    #[test]
    fn parses_a_scene_with_target_action() {
        let step: Step = parse(
            "say: Hi\npage: home\nselector: '#name'\nhighlight: true\nfill: Alex\ndelay_ms: 300\n",
        )
        .expect("valid step");
        assert_eq!(step.say, "Hi");
        assert_eq!(step.page.as_deref(), Some("home"));
        assert_eq!(step.action, Some(TargetAction::Fill("Alex".into())));
        assert!(step.highlight);
        assert_eq!(step.delay_ms, Some(300));
    }

    #[test]
    fn parses_optional_narration() {
        let step: Step = parse("say: Hello\nspeak: true\nposition: left\n").expect("valid");
        assert!(step.speak);
        assert_eq!(step.position, Some(NotificationPosition::Left));
        let default: Step = parse("say: Hello\n").expect("valid");
        assert!(!default.speak);
        assert_eq!(default.position, None);
        assert!(parse::<NoExtension>("say: Hello\nposition: middle\n").is_err());
        assert!(parse::<NoExtension>("include: child\nspeak: false\n").is_err());
    }

    #[test]
    fn rejects_unknown_keys_combined_actions_and_untargeted_actions() {
        assert!(parse::<NoExtension>("say: Hi\nteleport: true\n").is_err());
        assert!(parse::<NoExtension>("selector: a\nclick: true\ntap: true\n").is_err());
        assert!(parse::<NoExtension>("click: true\n").is_err());
        assert!(parse::<NoExtension>("include: child\nsay: Hi\n").is_err());
        assert_eq!(
            parse::<NoExtension>("include: child\n").expect("include"),
            Step::include("child")
        );
    }

    #[test]
    fn host_keys_decode_into_the_extension() {
        #[derive(Debug, Default, Deserialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        struct Music {
            midi: Option<Vec<u8>>,
        }
        let step: Step<Music> = parse("say: Listen\nmidi: [60, 64]\n").expect("valid");
        assert_eq!(step.extension.midi, Some(vec![60, 64]));
        assert!(parse::<Music>("say: Listen\nmidy: [60]\n").is_err());
        assert!(parse::<Music>("include: child\nmidi: [60]\n").is_err());
    }
}
