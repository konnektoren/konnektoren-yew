use crate::i18n::use_i18n;
use konnektoren_core::prelude::GamePath;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct SelectLevelCompProps {
    pub current: usize,
    pub levels: Vec<GamePath>,
    pub on_select: Callback<usize>,
}

#[function_component(SelectLevelComp)]
pub fn select_level(props: &SelectLevelCompProps) -> Html {
    let i18n = use_i18n();

    let on_change = {
        let on_select = props.on_select.clone();
        Callback::from(move |e: Event| {
            #[cfg(feature = "csr")]
            {
                use web_sys::HtmlSelectElement;
                if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                    let value = select.value();
                    let value = value.parse::<usize>().unwrap();
                    on_select.emit(value);
                }
            }
        })
    };

    html! {
        <div class="select-level">
            <label for="level-select">{ i18n.t("Select Level") }</label>
            <select id="level-select" onchange={on_change} value={props.current.to_string()}>
                <option value="" disabled={true} selected={props.levels.is_empty()}>
                    { i18n.t("Select Level") }
                </option>
                { for props.levels.iter().enumerate().map(|(index, level)| html! {
                    <option value={index.to_string()} selected={index == props.current}>
                        { i18n.t(&level.name) }
                    </option>
                }) }
            </select>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SelectLevelComp,
        SelectLevelCompProps::default(),
        (
            "Game Path",
            SelectLevelCompProps {
                current: 0,
                levels: vec![GamePath::default(), GamePath::default()],
                on_select: Callback::noop(),
            }
        ),
    );
}
