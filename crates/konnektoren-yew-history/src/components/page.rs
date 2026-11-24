use super::{History, HistoryContext};
use crate::model::{HistoryChange, HistoryQuery};
use konnektoren_yew_i18n::{I18nConfig, I18nContext};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

/// What the player changed on a page: the app's page state, query and paging.
pub(crate) struct PageState<H: History> {
    pub history: H,
    pub query: HistoryQuery<H::Filters, H::Sort>,
    /// Records currently shown.
    pub shown: usize,
}

impl<H: History> PageState<H> {
    /// Sets the query and goes back to the first page; `true` when it changed.
    pub fn set_query(&mut self, query: HistoryQuery<H::Filters, H::Sort>) -> bool {
        self.shown = H::PAGE;
        let changed = self.query != query;
        self.query = query;
        changed
    }
}

/// The state of a history page in a function component; see [`use_history_page`].
pub(crate) struct Page<H: History> {
    pub state: Rc<RefCell<PageState<H>>>,
    pub send: Rc<Callback<H::Message>>,
    i18n: Option<I18nContext>,
    fallback: Rc<I18nConfig>,
    rerender: UseForceUpdateHandle,
}

impl<H: History> Page<H> {
    /// The app's translations, or none (keys shown) without an i18n provider.
    pub fn i18n(&self) -> &I18nConfig {
        self.i18n
            .as_ref()
            .map_or(&*self.fallback, |context| &*context.config)
    }

    pub fn context<'a>(
        &'a self,
        props: &'a H::Properties,
        state: &'a PageState<H>,
    ) -> HistoryContext<'a, H> {
        HistoryContext {
            props,
            send: &self.send,
            i18n: self.i18n(),
            query: &state.query,
        }
    }

    /// A callback changing the state; renders again when `change` returns `true`.
    pub fn callback<IN: 'static>(
        &self,
        change: impl Fn(IN, &mut PageState<H>) -> bool + 'static,
    ) -> Callback<IN> {
        let state = self.state.clone();
        let rerender = self.rerender.clone();
        Callback::from(move |input| {
            if change(input, &mut state.borrow_mut()) {
                rerender.force_update();
            }
        })
    }
}

/// Holds the [`History`] like a struct component would: created once with
/// [`History::create`], app messages applied with [`History::update`] before the next
/// render, in the order they were sent.
#[hook]
pub(crate) fn use_history_page<H: History>(props: &H::Properties) -> Page<H> {
    let i18n = use_context::<I18nContext>();
    let fallback = use_memo((), |_| I18nConfig::default());
    let rerender = use_force_update();
    let pending = use_mut_ref(Vec::<H::Message>::new);
    let send = {
        let pending = pending.clone();
        let rerender = rerender.clone();
        use_memo((), move |_| {
            Callback::from(move |msg| {
                pending.borrow_mut().push(msg);
                rerender.force_update();
            })
        })
    };
    let config = i18n.as_ref().map_or(&*fallback, |context| &*context.config);
    let state = use_mut_ref(|| {
        let query = HistoryQuery::default();
        let history = H::create(&HistoryContext {
            props,
            send: &send,
            i18n: config,
            query: &query,
        });
        PageState {
            history,
            query,
            shown: H::PAGE,
        }
    });
    // Messages sent while updating are applied in the same pass.
    loop {
        let messages = std::mem::take(&mut *pending.borrow_mut());
        if messages.is_empty() {
            break;
        }
        let mut page = state.borrow_mut();
        let PageState {
            history,
            query,
            shown,
        } = &mut *page;
        for msg in messages {
            let context = HistoryContext {
                props,
                send: &send,
                i18n: config,
                query,
            };
            if history.update(&context, msg) == HistoryChange::Refilter {
                *shown = H::PAGE;
            }
        }
    }
    Page {
        state,
        send,
        i18n,
        fallback,
        rerender,
    }
}
