use super::PlaybackStep;
use serde::Deserialize;

/// A named excursion containing a sequence of playback instructions.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Excursion {
    pub version: u32,
    pub id: String,
    pub title: String,
    pub steps: Vec<PlaybackStep>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_example_excursion() {
        let script: Excursion =
            serde_yaml::from_str(include_str!("../../examples/introduction.yaml"))
                .expect("example excursion should be valid");
        assert_eq!(script.version, 1);
        assert_eq!(script.id, "introduction");
        assert_eq!(script.steps.len(), 11);
        assert_eq!(
            script.steps[0],
            PlaybackStep::Say {
                say: "Welcome to the demonstration.".into()
            }
        );
        assert!(matches!(&script.steps[1], PlaybackStep::Goto { goto } if goto == "/challenges"));
        assert!(matches!(&script.steps[2], PlaybackStep::Zoom { zoom } if *zoom == 1.1));
        assert!(
            matches!(&script.steps[3], PlaybackStep::Scroll { scroll } if scroll.x == 0.0 && scroll.y == 300.0)
        );
        assert!(matches!(&script.steps[6], PlaybackStep::Fill { fill } if fill.value == "grammar"));
        assert!(
            matches!(&script.steps[8], PlaybackStep::ScrollIntoView { scroll_into_view } if scroll_into_view.contains("challenge-card"))
        );
    }

    #[test]
    fn rejects_unknown_instructions() {
        let yaml = "version: 1\nid: demo\ntitle: Demo\nsteps:\n  - teleport: '#button'\n";
        assert!(serde_yaml::from_str::<Excursion>(yaml).is_err());
    }

    #[test]
    fn rejects_steps_with_multiple_actions() {
        let yaml =
            "version: 1\nid: demo\ntitle: Demo\nsteps:\n  - say: hello\n    click: '#button'\n";
        assert!(serde_yaml::from_str::<Excursion>(yaml).is_err());
    }

    #[test]
    fn application_can_decode_extension_parameters() {
        #[derive(serde::Deserialize)]
        struct MusicAction {
            notes: Vec<Note>,
        }
        #[derive(serde::Deserialize)]
        struct Note {
            pitch: u8,
            duration_ms: u32,
            gap_ms: u32,
        }

        let excursion: Excursion =
            serde_yaml::from_str(include_str!("../../examples/extension.yaml"))
                .expect("extension example should parse");
        let PlaybackStep::Extension(extension) = &excursion.steps[1] else {
            panic!("expected extension")
        };
        assert_eq!(extension.name, "music.play_notes");
        let action: MusicAction =
            serde_json::from_value(extension.params.clone()).expect("host parameters should parse");
        assert_eq!(action.notes.len(), 2);
        assert_eq!(
            (
                action.notes[0].pitch,
                action.notes[0].duration_ms,
                action.notes[0].gap_ms
            ),
            (60, 300, 100)
        );
    }

    #[test]
    fn rejects_multiple_or_malformed_extension_actions() {
        for step in [
            "  - music.play: {}\n    music.stop: {}",
            "  - .play: {}",
            "  - music.: {}",
        ] {
            let yaml = format!("version: 1\nid: demo\ntitle: Demo\nsteps:\n{step}\n");
            assert!(serde_yaml::from_str::<Excursion>(&yaml).is_err());
        }
    }
}
