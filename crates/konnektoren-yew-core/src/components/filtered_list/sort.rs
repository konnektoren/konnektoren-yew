use crate::model::ListSort;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListSortFieldProps {
    pub sort: ListSort,
    /// Translated, e.g. "Sort by".
    pub label: AttrValue,
    pub on_change: Callback<String>,
}

/// The sort select of a [`FilteredList`](super::FilteredList); nothing with fewer than
/// two orders.
#[function_component(ListSortField)]
pub fn list_sort_field(props: &ListSortFieldProps) -> Html {
    let sort = &props.sort;
    if sort.choices.len() < 2 {
        return Html::default();
    }
    let onchange = props.on_change.reform(|event: Event| {
        event
            .target_unchecked_into::<web_sys::HtmlSelectElement>()
            .value()
    });
    html! {
        <label class="filtered-list__field">
            <span class="filtered-list__label">{props.label.clone()}</span>
            <select class="filtered-list__select" {onchange}>
                {for sort.choices.iter().map(|choice| html! {
                    <option value={choice.value.clone()} selected={choice.value == sort.value}>{choice.label.clone()}</option>
                })}
            </select>
        </label>
    }
}
