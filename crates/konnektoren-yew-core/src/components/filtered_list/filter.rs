use crate::model::{ListFilter, ListFilterControl};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListFilterFieldProps {
    pub filter: ListFilter,
    /// Translated label above toggles, e.g. "Show".
    #[prop_or(AttrValue::Static("Show"))]
    pub show: AttrValue,
    /// `(filter id, choice value)`.
    pub on_change: Callback<(AttrValue, AttrValue)>,
}

/// One filter of a [`FilteredList`](super::FilteredList): a select, or a toggle for
/// two choices. A toggle that is on stays enabled so it can be turned off.
#[function_component(ListFilterField)]
pub fn list_filter_field(props: &ListFilterFieldProps) -> Html {
    let filter = &props.filter;
    let id = filter.id.clone();
    match &filter.control {
        ListFilterControl::Toggle { icon } if filter.choices.len() == 2 => {
            let on = filter.value == filter.choices[1].value;
            let next = filter.choices[usize::from(!on)].value.clone();
            let onclick = props
                .on_change
                .reform(move |_: MouseEvent| (id.clone(), next.clone()));
            html! {
                <div class="filtered-list__field">
                    <span class="filtered-list__label" aria-hidden="true">{props.show.clone()}</span>
                    <button type="button"
                        class={classes!("filtered-list__toggle", on.then_some("filtered-list__toggle--on"))}
                        aria-pressed={on.to_string()} disabled={filter.disabled && !on} {onclick}>
                        <i class={classes!("filtered-list__toggle-icon", if on { "fa-solid" } else { "fa-regular" }, icon.to_string())} aria-hidden="true"></i>
                        {filter.choices[1].label.clone()}
                    </button>
                </div>
            }
        }
        _ => {
            let onchange = props.on_change.reform(move |event: Event| {
                let value = event
                    .target_unchecked_into::<web_sys::HtmlSelectElement>()
                    .value();
                (id.clone(), AttrValue::from(value))
            });
            html! {
                <label class="filtered-list__field">
                    <span class="filtered-list__label">{filter.label.clone()}</span>
                    <select class="filtered-list__select" disabled={filter.disabled} {onchange}>
                        {for filter.choices.iter().map(|choice| html! {
                            <option value={choice.value.clone()} selected={choice.value == filter.value}>
                                {choice.label.clone()}
                            </option>
                        })}
                    </select>
                </label>
            }
        }
    }
}
