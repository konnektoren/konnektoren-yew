use super::{History, HistoryContext, HistoryEntryRow};
use crate::model::{HistoryItem, TimeValue, Tone};
use konnektoren_yew_core::signed;
use std::ops::Deref;
use yew::prelude::*;

/// What one row can use: the page context plus the row's position, translated name
/// and the page state.
pub struct EntryContext<'a, H: History> {
    /// Position in the stored history.
    pub index: usize,
    /// Translated name, as searched.
    pub name: AttrValue,
    pub(crate) history: &'a H,
    pub(crate) base: &'a HistoryContext<'a, H>,
}

impl<'a, H: History> EntryContext<'a, H> {
    /// The page state, e.g. the selected row.
    pub fn history(&self) -> &'a H {
        self.history
    }
}

impl<'a, H: History> Deref for EntryContext<'a, H> {
    type Target = HistoryContext<'a, H>;

    fn deref(&self) -> &Self::Target {
        self.base
    }
}

/// A record that renders its own row. Implement this only when the standard row of
/// [`DefaultHistoryEntry`] does not fit; a type implements one of the two per history.
pub trait HistoryEntry<H: History>: HistoryItem {
    fn view(&self, ctx: &EntryContext<'_, H>) -> Html;

    /// In [`HistoryPreview`](super::HistoryPreview).
    fn view_compact(&self, ctx: &EntryContext<'_, H>) -> Html {
        self.view(ctx)
    }
}

/// A record shown in the standard row: the app says *what* to show, the crate how.
/// Every method has a default, so an empty impl shows the name and time.
pub trait DefaultHistoryEntry<H: History>: HistoryItem {
    fn title(&self, ctx: &EntryContext<'_, H>) -> AttrValue {
        ctx.name.clone()
    }

    /// Before the title, e.g. an emoji.
    fn icon(&self, _ctx: &EntryContext<'_, H>) -> Option<Html> {
        None
    }

    fn selected(&self, _ctx: &EntryContext<'_, H>) -> bool {
        false
    }

    /// Makes the title a button.
    fn on_select(&self, _ctx: &EntryContext<'_, H>) -> Option<Callback<MouseEvent>> {
        None
    }

    /// Small facts, e.g. the opponent's rating or the rating change.
    fn meta(&self, _ctx: &EntryContext<'_, H>) -> Vec<EntryMeta> {
        Vec::new()
    }

    /// The main badge, e.g. "80%" or "You win".
    fn result(&self, _ctx: &EntryContext<'_, H>) -> Option<EntryResult> {
        None
    }

    /// Buttons, e.g. replay and favourite.
    fn actions(&self, _ctx: &EntryContext<'_, H>) -> Html {
        Html::default()
    }

    fn entry_time(&self, _ctx: &EntryContext<'_, H>) -> Option<TimeValue> {
        self.time().map(TimeValue::At)
    }

    fn parts(&self, ctx: &EntryContext<'_, H>) -> EntryParts {
        EntryParts {
            icon: self.icon(ctx),
            title: self.title(ctx),
            selected: self.selected(ctx),
            on_select: self.on_select(ctx),
            meta: self.meta(ctx),
            result: self.result(ctx),
            actions: self.actions(ctx),
            time: self.entry_time(ctx),
        }
    }
}

impl<H: History, T: DefaultHistoryEntry<H>> HistoryEntry<H> for T {
    fn view(&self, ctx: &EntryContext<'_, H>) -> Html {
        html! { <HistoryEntryRow parts={self.parts(ctx)} /> }
    }

    fn view_compact(&self, ctx: &EntryContext<'_, H>) -> Html {
        html! { <HistoryEntryRow parts={self.parts(ctx)} compact=true /> }
    }
}

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
