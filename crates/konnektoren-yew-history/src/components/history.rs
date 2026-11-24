use super::HistoryEntry;
use crate::model::{
    FilterContext, HistoryChange, HistoryFilters, HistoryLabels, HistoryQuery, HistorySort,
};
use konnektoren_yew_i18n::I18nConfig;
use yew::prelude::*;

/// A history page, written like a Yew struct component. The crate's
/// [`HistoryView`](crate::HistoryView) holds the search, filters, sort and paging and
/// asks this type for everything specific to the page.
///
/// ```ignore
/// html! { <HistoryView<ExerciseHistory> history={history} /> }
/// ```
pub trait History: Sized + 'static {
    /// One record; shown through [`HistoryEntry`] or
    /// [`DefaultHistoryEntry`](super::DefaultHistoryEntry).
    type Item: HistoryEntry<Self>;
    /// `()` or a tuple of filters.
    type Filters: HistoryFilters<Self>;
    /// The sort orders, e.g. [`TimeSort`](crate::model::TimeSort).
    type Sort: HistorySort<Self>;
    /// Props of `HistoryView<Self>`: the data and the app's callbacks.
    type Properties: Properties;
    /// App messages, e.g. select a row.
    type Message: 'static;

    /// Records per "Show more".
    const PAGE: usize = 20;
    /// Extra class on the page root.
    const CLASS: &'static str = "";

    /// Like `Component::create`.
    fn create(ctx: &HistoryContext<'_, Self>) -> Self;

    /// Records in stored order, or why they could not be read. Called on create and
    /// when the props change.
    fn items(props: &Self::Properties) -> Result<Vec<Self::Item>, AttrValue>;

    fn labels() -> HistoryLabels;

    /// Filter on page state that is not a [`HistoryFilter`](crate::model::HistoryFilter).
    fn admits(&self, _ctx: &FilterContext<'_, Self>, _item: &Self::Item) -> bool {
        true
    }

    /// Like `Component::update`.
    fn update(&mut self, _ctx: &HistoryContext<'_, Self>, _msg: Self::Message) -> HistoryChange {
        HistoryChange::Render
    }

    /// Heading and intro.
    fn view_header(&self, ctx: &HistoryContext<'_, Self>) -> Html {
        let labels = Self::labels();
        html! {
            <header class="history-page__heading">
                <h1 class="history-page__title">{ctx.t(labels.title)}</h1>
                if !labels.intro.is_empty() { <p class="history-page__intro">{ctx.t(labels.intro)}</p> }
            </header>
        }
    }

    /// Above the filters, e.g. rating and wins. `items` are all records.
    fn view_summary(&self, _ctx: &HistoryContext<'_, Self>, _items: &[Self::Item]) -> Html {
        Html::default()
    }

    /// Controls in the filter form that are not filters.
    fn view_controls(&self, _ctx: &HistoryContext<'_, Self>) -> Html {
        Html::default()
    }

    /// A panel beside the list, e.g. the attempts of the selected exercise.
    fn view_aside(&self, _ctx: &HistoryContext<'_, Self>) -> Option<Html> {
        None
    }

    /// Below the list.
    fn view_footer(&self, _ctx: &HistoryContext<'_, Self>) -> Html {
        Html::default()
    }

    /// Next to the empty message, e.g. a link to the exercises.
    fn view_empty_action(&self, _ctx: &HistoryContext<'_, Self>) -> Html {
        Html::default()
    }
}

/// What a [`History`] can use while creating, updating and rendering.
pub struct HistoryContext<'a, H: History> {
    pub(crate) props: &'a H::Properties,
    pub(crate) send: &'a Callback<H::Message>,
    pub(crate) i18n: &'a I18nConfig,
    pub(crate) query: &'a HistoryQuery<H::Filters, H::Sort>,
}

impl<'a, H: History> HistoryContext<'a, H> {
    pub fn props(&self) -> &'a H::Properties {
        self.props
    }

    /// Translate.
    pub fn t(&self, key: &str) -> String {
        self.i18n.t(key)
    }

    pub fn query(&self) -> &'a HistoryQuery<H::Filters, H::Sort> {
        self.query
    }

    pub fn send_message(&self, msg: H::Message) {
        self.send.emit(msg);
    }

    /// A callback sending an app message, like `ctx.link().callback`.
    pub fn callback<IN: 'static>(&self, f: impl Fn(IN) -> H::Message + 'static) -> Callback<IN> {
        let send = self.send.clone();
        Callback::from(move |input| send.emit(f(input)))
    }
}
