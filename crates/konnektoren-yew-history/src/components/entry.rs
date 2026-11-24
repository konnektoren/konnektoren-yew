use super::{History, HistoryContext, HistoryEntryRow};
use crate::model::{HistoryItem, TimeValue};
use konnektoren_yew_core::{EntryMeta, EntryParts, EntryResult};
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
