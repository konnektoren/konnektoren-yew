use super::HistoryOption;
use crate::components::History;

/// What a filter can look at besides the record: the page props and state, e.g. the
/// replays that are favourites.
pub struct FilterContext<'a, H: History> {
    pub props: &'a H::Properties,
    pub history: &'a H,
}

/// How a filter is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterControl {
    Select,
    /// A button with `aria-pressed` for a filter with two choices: off is the default,
    /// on the other one. `icon` is a Font Awesome name like `fa-star`.
    Toggle {
        icon: &'static str,
    },
}

/// One filter: decides whether a record is listed.
pub trait HistoryFilter<H: History>: HistoryOption {
    /// Form and URL name, e.g. `result`.
    const ID: &'static str;
    /// Translation key of the control's label.
    const LABEL: &'static str;
    const CONTROL: FilterControl = FilterControl::Select;
    /// Show how many records each choice would list, e.g. "Wins (12)".
    const COUNTS: bool = false;

    fn admits(self, item: &H::Item, ctx: &FilterContext<'_, H>) -> bool;

    /// Disable the control, e.g. while there are no favourites.
    fn disabled(_ctx: &FilterContext<'_, H>) -> bool {
        false
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterChoice {
    pub value: &'static str,
    pub label: &'static str,
}

/// A filter as the form shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterField {
    pub id: &'static str,
    pub label: &'static str,
    pub control: FilterControl,
    /// The chosen value.
    pub value: &'static str,
    pub choices: Vec<FilterChoice>,
    pub counts: bool,
    pub disabled: bool,
}

impl FilterField {
    fn of<H: History, F: HistoryFilter<H>>(filter: F, ctx: &FilterContext<'_, H>) -> Self {
        Self {
            id: F::ID,
            label: F::LABEL,
            control: F::CONTROL,
            value: filter.value(),
            choices: F::ALL
                .iter()
                .map(|choice| FilterChoice {
                    value: choice.value(),
                    label: choice.label(),
                })
                .collect(),
            counts: F::COUNTS,
            disabled: F::disabled(ctx),
        }
    }
}

/// All filters of a history: `()` for none, or a tuple of up to four
/// [`HistoryFilter`]s like `(Results,)` or `(Outcomes, Favourites)`.
pub trait HistoryFilters<H: History>: Clone + PartialEq + Default + 'static {
    /// Every filter admits the record.
    fn admits(&self, item: &H::Item, ctx: &FilterContext<'_, H>) -> bool;
    /// Any filter is not at its default.
    fn active(&self) -> bool;
    fn fields(&self, ctx: &FilterContext<'_, H>) -> Vec<FilterField>;
    /// Sets the filter `id` to the choice `value`; `false` when there is no such filter.
    fn set(&mut self, id: &str, value: &str) -> bool;
}

impl<H: History> HistoryFilters<H> for () {
    fn admits(&self, _: &H::Item, _: &FilterContext<'_, H>) -> bool {
        true
    }
    fn active(&self) -> bool {
        false
    }
    fn fields(&self, _: &FilterContext<'_, H>) -> Vec<FilterField> {
        Vec::new()
    }
    fn set(&mut self, _: &str, _: &str) -> bool {
        false
    }
}

macro_rules! tuple_filters {
    ($($filter:ident $index:tt),+) => {
        impl<H: History, $($filter: HistoryFilter<H>),+> HistoryFilters<H> for ($($filter,)+) {
            fn admits(&self, item: &H::Item, ctx: &FilterContext<'_, H>) -> bool {
                true $(&& self.$index.admits(item, ctx))+
            }
            fn active(&self) -> bool {
                false $(|| self.$index != $filter::default())+
            }
            fn fields(&self, ctx: &FilterContext<'_, H>) -> Vec<FilterField> {
                vec![$(FilterField::of(self.$index, ctx)),+]
            }
            fn set(&mut self, id: &str, value: &str) -> bool {
                $(
                    if id == <$filter as HistoryFilter<H>>::ID {
                        self.$index = <$filter as HistoryOption>::parse(value);
                        return true;
                    }
                )+
                false
            }
        }
    };
}

tuple_filters!(A 0);
tuple_filters!(A 0, B 1);
tuple_filters!(A 0, B 1, C 2);
tuple_filters!(A 0, B 1, C 2, D 3);
