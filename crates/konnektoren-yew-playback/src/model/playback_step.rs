use serde::Deserialize;

/// An instruction in an excursion. Element targets are CSS selectors.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum PlaybackStep {
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
