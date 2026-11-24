use super::{NoExtension, Step};
use serde::Deserialize;
use serde::de::DeserializeOwned;

/// A named excursion: a sequence of steps, optionally looping.
///
/// `X` is the host's step extension, see [`Step`].
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, bound(deserialize = "X: DeserializeOwned"))]
pub struct Excursion<X = NoExtension> {
    pub version: u32,
    pub id: String,
    pub title: String,
    /// Restart at the first step after the last step completes.
    #[serde(default)]
    pub repeat: bool,
    /// Optional image shown alongside the playback cursor.
    #[serde(default)]
    pub cursor: Option<CursorConfig>,
    pub steps: Vec<Step<X>>,
}

/// Presentation settings for an excursion's cursor.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CursorConfig {
    pub image: String,
}

impl<X> Excursion<X> {
    /// The page in effect at `index`: the latest `page` set at or before it.
    pub fn page_at(&self, index: usize) -> Option<&str> {
        self.steps
            .iter()
            .take(index.saturating_add(1))
            .rev()
            .find_map(|step| step.page.as_deref())
    }

    /// Ids of excursions included directly by this one, in order.
    pub fn includes(&self) -> impl Iterator<Item = &str> {
        self.steps.iter().filter_map(|step| step.include.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TargetAction;

    #[test]
    fn parses_the_example_excursion() {
        let script: Excursion = serde_yaml::from_str(include_str!("../../examples/preview.yaml"))
            .expect("example excursion should be valid");
        assert_eq!(script.id, "preview");
        assert!(!script.repeat);
        assert_eq!(script.page_at(0), None);
        assert!(script.steps.iter().any(|step| step.highlight));
    }

    #[test]
    fn parses_repeat_cursor_and_pages() {
        let yaml = "version: 1\nid: loop\ntitle: Loop\nrepeat: true\ncursor:\n  image: /m.svg\nsteps:\n  - say: a\n    page: home\n  - say: b\n    selector: a\n    tap: true\n  - say: c\n    page: duet\n";
        let excursion: Excursion = serde_yaml::from_str(yaml).expect("valid excursion");
        assert!(excursion.repeat);
        assert_eq!(
            excursion.cursor.as_ref().map(|c| c.image.as_str()),
            Some("/m.svg")
        );
        assert_eq!(excursion.steps[1].action, Some(TargetAction::Tap));
        assert_eq!(excursion.page_at(1), Some("home"));
        assert_eq!(excursion.page_at(9), Some("duet"));
    }

    #[test]
    fn rejects_unknown_keys() {
        let yaml = "version: 1\nid: demo\ntitle: Demo\nsteps:\n  - teleport: '#button'\n";
        assert!(serde_yaml::from_str::<Excursion>(yaml).is_err());
        let yaml = "version: 1\nid: demo\ntitle: Demo\ncursor:\n  img: x\nsteps: []\n";
        assert!(serde_yaml::from_str::<Excursion>(yaml).is_err());
    }

    #[test]
    fn host_extensions_parse_from_the_example() {
        #[derive(Debug, Default, Deserialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        struct Music {
            midi: Option<Vec<Note>>,
        }
        #[derive(Debug, Deserialize, PartialEq)]
        struct Note {
            pitch: u8,
            duration_ms: u32,
            gap_ms: u32,
        }
        let excursion: Excursion<Music> =
            serde_yaml::from_str(include_str!("../../examples/extension.yaml"))
                .expect("extension example should parse");
        let notes = excursion.steps[1].extension.midi.as_ref().expect("notes");
        assert_eq!(
            notes[0],
            Note {
                pitch: 60,
                duration_ms: 300,
                gap_ms: 100
            }
        );
        assert!(
            serde_yaml::from_str::<Excursion>(include_str!("../../examples/extension.yaml"))
                .is_err()
        );
    }
}
