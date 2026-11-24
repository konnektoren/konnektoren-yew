use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListCountProps {
    pub matching: usize,
    pub total: usize,
    /// Translated, e.g. "Attempts".
    pub noun: AttrValue,
    /// Shows the clear button.
    #[prop_or_default]
    pub active: bool,
    /// Translated, e.g. "Clear filters".
    pub clear_label: AttrValue,
    pub on_clear: Callback<()>,
}

/// "matching / total noun", announced politely, with "Clear filters" while a search
/// or filter is set.
#[function_component(ListCount)]
pub fn list_count(props: &ListCountProps) -> Html {
    html! {
        <p class="filtered-list__count" role="status" aria-live="polite">
            {format!("{} / {} ", props.matching, props.total)}{props.noun.clone()}
            if props.active {
                <button type="button" class="filtered-list__clear" onclick={props.on_clear.reform(|_: MouseEvent| ())}>
                    <i class="filtered-list__clear-icon fa-solid fa-xmark" aria-hidden="true"></i>{props.clear_label.clone()}
                </button>
            }
        </p>
    }
}
