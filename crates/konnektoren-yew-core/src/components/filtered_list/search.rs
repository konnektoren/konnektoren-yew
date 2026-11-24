use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListSearchProps {
    /// Translated.
    pub label: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    pub value: AttrValue,
    pub on_input: Callback<String>,
}

/// The search field of a [`FilteredList`](super::FilteredList).
#[function_component(ListSearch)]
pub fn list_search(props: &ListSearchProps) -> Html {
    let oninput = props.on_input.reform(|event: InputEvent| {
        event
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value()
    });
    html! {
        <label class="filtered-list__field filtered-list__field--search">
            <span class="filtered-list__label">{props.label.clone()}</span>
            <span class="filtered-list__search">
                <i class="filtered-list__search-icon fa-solid fa-magnifying-glass" aria-hidden="true"></i>
                <input type="search" class="filtered-list__search-input" value={props.value.clone()}
                    placeholder={props.placeholder.clone()} {oninput} />
            </span>
        </label>
    }
}
