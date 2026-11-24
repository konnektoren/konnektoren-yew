use serde::Deserialize;

/// An instruction in an excursion. Element targets are CSS selectors.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum PlaybackStep {
    /// Expand the named excursion before playback.
    Include { include: String },
    /// Display a message, translated by the host application.
    Say { say: String },
    /// Navigate to an application path.
    Goto { goto: String },
    /// Wait for an element matching the selector.
    WaitFor { wait_for: String },
    /// Visually highlight elements matching the selector.
    Highlight { highlight: String },
    /// Click an element matching the selector.
    Click { click: String },
    /// Fill an input element.
    Fill { fill: FillAction },
    /// Send a key to an element.
    Press { press: PressAction },
    /// Scroll the viewport to document coordinates in CSS pixels.
    Scroll { scroll: ScrollAction },
    /// Scroll a matching element into view.
    ScrollIntoView { scroll_into_view: String },
    /// Set the application content scale (1.0 is normal), not browser zoom.
    Zoom { zoom: f64 },
    /// An application-defined action interpreted by the playback host.
    Extension(ExtensionAction),
}

/// An extension name and JSON-compatible data owned by the host application.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionAction {
    pub name: String,
    pub params: serde_json::Value,
}

impl<'de> Deserialize<'de> for ExtensionAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        let entries = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        if entries.len() != 1 {
            return Err(D::Error::custom(
                "an extension step must contain exactly one action",
            ));
        }
        let (name, params) = entries
            .into_iter()
            .next()
            .ok_or_else(|| D::Error::custom("missing extension action"))?;
        if !name.contains('.') || name.split('.').any(str::is_empty) {
            return Err(D::Error::custom(
                "extension names must be namespaced, such as music.play_notes",
            ));
        }
        Ok(Self { name, params })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FillAction {
    pub selector: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PressAction {
    pub selector: String,
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScrollAction {
    pub x: f64,
    pub y: f64,
}
