use super::{TimeValue, Tone, signed};
use yew::prelude::*;

/// A small fact in a row.
#[derive(Clone, Default, PartialEq)]
pub struct EntryMeta {
    pub icon: Option<Html>,
    pub text: AttrValue,
    /// Tooltip, e.g. "Opponent’s rating".
    pub title: Option<AttrValue>,
    pub tone: Tone,
}

impl EntryMeta {
    pub fn text(text: impl Into<AttrValue>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    pub fn with_icon(mut self, icon: Html) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_title(mut self, title: impl Into<AttrValue>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    /// Rounded rating change like `+12` or `−8`, with `before → after` as tooltip.
    pub fn rating_change(before: f64, after: f64) -> Self {
        let change = after.round() - before.round();
        let tone = if change > 0.0 {
            Tone::Good
        } else if change < 0.0 {
            Tone::Bad
        } else {
            Tone::Neutral
        };
        Self::text(signed(change))
            .with_title(format!("{before:.0} → {after:.0}"))
            .with_tone(tone)
    }
}

/// The main badge of a row.
#[derive(Clone, Default, PartialEq)]
pub struct EntryResult {
    pub icon: Option<Html>,
    pub text: AttrValue,
    /// Accessible name, e.g. "Result".
    pub label: Option<AttrValue>,
    pub tone: Tone,
}

impl EntryResult {
    pub fn new(text: impl Into<AttrValue>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    pub fn with_icon(mut self, icon: Html) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_label(mut self, label: impl Into<AttrValue>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    /// A percentage: good from `strong_from`, a warning below `weak_below`.
    pub fn score(score: u32, strong_from: u32, weak_below: u32) -> Self {
        let tone = if score >= strong_from {
            Tone::Good
        } else if score < weak_below {
            Tone::Warn
        } else {
            Tone::Neutral
        };
        Self::new(format!("{score}%")).with_tone(tone)
    }
}

/// Everything the standard row shows.
#[derive(Clone, Default, PartialEq)]
pub struct EntryParts {
    pub icon: Option<Html>,
    pub title: AttrValue,
    pub selected: bool,
    pub on_select: Option<Callback<MouseEvent>>,
    pub meta: Vec<EntryMeta>,
    pub result: Option<EntryResult>,
    pub actions: Html,
    pub time: Option<TimeValue>,
}
