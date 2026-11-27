use crate::i18n::use_i18n;
use konnektoren_core::prelude::GamePath;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct SelectLevelCompProps {
    pub current: usize,
    pub levels: Vec<GamePath>,
    pub on_select: Callback<usize>,
}

const SELECT_ID: &str = "level-select";
const LABEL_ID: &str = "level-select-label";
const LISTBOX_ID: &str = "level-select-listbox";

fn option_id(index: usize) -> String {
    format!("level-select-option-{index}")
}

/// Picks a level. Levels with an icon or a description get a picker that shows the icon
/// in front of each name and the description of the option under the pointer or the
/// keyboard highlight. Without either, it stays a native `<select>`.
#[function_component(SelectLevelComp)]
pub fn select_level(props: &SelectLevelCompProps) -> Html {
    let rich = props
        .levels
        .iter()
        .any(|level| level.icon.is_some() || level.description.is_some());
    if rich {
        html! { <RichSelectLevel current={props.current} levels={props.levels.clone()} on_select={props.on_select.clone()} /> }
    } else {
        html! { <NativeSelectLevel current={props.current} levels={props.levels.clone()} on_select={props.on_select.clone()} /> }
    }
}

#[function_component(NativeSelectLevel)]
fn native_select_level(props: &SelectLevelCompProps) -> Html {
    let i18n = use_i18n();

    let on_change = {
        let on_select = props.on_select.clone();
        Callback::from(move |e: Event| {
            #[cfg(feature = "csr")]
            {
                use web_sys::HtmlSelectElement;
                if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                    if let Ok(value) = select.value().parse::<usize>() {
                        on_select.emit(value);
                    }
                }
            }
            #[cfg(not(feature = "csr"))]
            let _ = (e, &on_select);
        })
    };

    html! {
        <div class="select-level">
            <label for={SELECT_ID}>{ i18n.t("Select Level") }</label>
            <select id={SELECT_ID} onchange={on_change} value={props.current.to_string()}>
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

fn level_icon(level: Option<&GamePath>) -> Html {
    match level.and_then(|level| level.icon.as_deref()) {
        Some(src) => html! {
            <img class="select-level__icon" src={src.to_owned()} alt="" aria-hidden="true" />
        },
        None => html! {
            <span class="select-level__icon select-level__icon--empty" aria-hidden="true"></span>
        },
    }
}

/// Select-only combobox: focus stays on the button, `aria-activedescendant` names the
/// highlighted option.
#[function_component(RichSelectLevel)]
fn rich_select_level(props: &SelectLevelCompProps) -> Html {
    let i18n = use_i18n();
    let open = use_state(|| false);
    let highlighted = use_state(|| props.current);
    let container = use_node_ref();
    let listbox = use_node_ref();
    let count = props.levels.len();
    let selected = props.levels.get(props.current);

    // Keeps the highlighted option visible while moving through a long list.
    {
        let listbox = listbox.clone();
        use_effect_with((*open, *highlighted), move |(open, highlighted)| {
            #[cfg(feature = "csr")]
            if *open {
                use wasm_bindgen::JsCast;
                if let Some(list) = listbox.cast::<web_sys::HtmlElement>() {
                    if let Some(option) = list
                        .children()
                        .item(*highlighted as u32)
                        .and_then(|option| option.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        let top = option.offset_top();
                        let bottom = top + option.offset_height();
                        if top < list.scroll_top() {
                            list.set_scroll_top(top);
                        } else if bottom > list.scroll_top() + list.client_height() {
                            list.set_scroll_top(bottom - list.client_height());
                        }
                    }
                }
            }
            #[cfg(not(feature = "csr"))]
            let _ = (listbox, open, highlighted);
            || ()
        });
    }

    let set_open = {
        let open = open.clone();
        let highlighted = highlighted.clone();
        let current = props.current;
        move |value: bool| {
            if value && !*open {
                highlighted.set(current.min(count.saturating_sub(1)));
            }
            open.set(value);
        }
    };

    let choose = {
        let on_select = props.on_select.clone();
        let open = open.clone();
        move |index: usize| {
            open.set(false);
            on_select.emit(index);
        }
    };

    let on_toggle = {
        let set_open = set_open.clone();
        let open = open.clone();
        Callback::from(move |_: MouseEvent| set_open(!*open))
    };

    let on_keydown = {
        let open = open.clone();
        let highlighted = highlighted.clone();
        let set_open = set_open.clone();
        let choose = choose.clone();
        Callback::from(move |e: KeyboardEvent| {
            if count == 0 {
                return;
            }
            let last = count - 1;
            let key = e.key();
            if !*open {
                if matches!(key.as_str(), "ArrowDown" | "ArrowUp" | "Enter" | " ") {
                    e.prevent_default();
                    set_open(true);
                }
                return;
            }
            match key.as_str() {
                "ArrowDown" => highlighted.set((*highlighted + 1).min(last)),
                "ArrowUp" => highlighted.set(highlighted.saturating_sub(1)),
                "Home" => highlighted.set(0),
                "End" => highlighted.set(last),
                "Enter" | " " => choose(*highlighted),
                "Escape" => open.set(false),
                "Tab" => {
                    open.set(false);
                    return;
                }
                _ => return,
            }
            e.prevent_default();
        })
    };

    // Closes when focus moves outside the picker, e.g. by clicking elsewhere.
    let on_focusout = {
        let open = open.clone();
        let container = container.clone();
        Callback::from(move |e: FocusEvent| {
            #[cfg(feature = "csr")]
            {
                use wasm_bindgen::JsCast;
                let inside = e
                    .related_target()
                    .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                    .zip(container.cast::<web_sys::Node>())
                    .is_some_and(|(target, container)| container.contains(Some(&target)));
                if !inside {
                    open.set(false);
                }
            }
            #[cfg(not(feature = "csr"))]
            let _ = (e, &open, &container);
        })
    };

    // Clicking an option must not take focus from the button, which would close the list.
    let keep_focus = Callback::from(|e: MouseEvent| e.prevent_default());

    let trigger_label = selected
        .map(|level| i18n.t(&level.name))
        .unwrap_or_else(|| i18n.t("Select Level"));
    let active_descendant = (*open && count > 0).then(|| option_id(*highlighted));

    let options = props.levels.iter().enumerate().map(|(index, level)| {
        let is_selected = index == props.current;
        let is_highlighted = index == *highlighted;
        let onclick = {
            let choose = choose.clone();
            Callback::from(move |_: MouseEvent| choose(index))
        };
        let onmouseenter = {
            let highlighted = highlighted.clone();
            Callback::from(move |_: MouseEvent| highlighted.set(index))
        };
        let description = level.description.as_ref().map(|description| {
            html! {
                <span class="select-level__description">{ i18n.t(description) }</span>
            }
        });
        html! {
            <li
                id={option_id(index)}
                role="option"
                aria-selected={is_selected.to_string()}
                class={classes!(
                    "select-level__option",
                    is_selected.then_some("select-level__option--selected"),
                    is_highlighted.then_some("select-level__option--highlighted"),
                )}
                {onclick}
                {onmouseenter}
            >
                { level_icon(Some(level)) }
                <span class="select-level__text">
                    <span class="select-level__name">{ i18n.t(&level.name) }</span>
                    { description }
                </span>
            </li>
        }
    });

    html! {
        <div
            class={classes!("select-level", "select-level--rich", open.then_some("select-level--open"))}
            ref={container}
            onfocusout={on_focusout}
        >
            <label id={LABEL_ID} for={SELECT_ID}>{ i18n.t("Select Level") }</label>
            <div class="select-level__control">
                <button
                    id={SELECT_ID}
                    type="button"
                    role="combobox"
                    class="select-level__trigger"
                    aria-labelledby={LABEL_ID}
                    aria-haspopup="listbox"
                    aria-controls={LISTBOX_ID}
                    aria-expanded={open.to_string()}
                    aria-activedescendant={active_descendant}
                    disabled={count == 0}
                    onclick={on_toggle}
                    onkeydown={on_keydown}
                >
                    { level_icon(selected) }
                    <span class="select-level__name">{ trigger_label }</span>
                    <span class="select-level__caret" aria-hidden="true"></span>
                </button>
                <ul
                    id={LISTBOX_ID}
                    role="listbox"
                    class="select-level__options"
                    aria-labelledby={LABEL_ID}
                    hidden={!*open}
                    ref={listbox}
                    onmousedown={keep_focus}
                >
                    { for options }
                </ul>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn level(id: &str, name: &str, description: Option<&str>, icon: Option<&str>) -> GamePath {
        GamePath {
            id: id.into(),
            name: name.into(),
            description: description.map(Into::into),
            icon: icon.map(Into::into),
            ..GamePath::default()
        }
    }

    fn rich_levels() -> Vec<GamePath> {
        vec![
            level(
                "a1",
                "Level A1",
                Some("First steps: everyday connectors such as und, aber and oder."),
                Some("/assets/challenges/images/level-a1.svg"),
            ),
            level(
                "a2",
                "Level A2",
                Some("Building sentences: join clauses with weil, dass and wenn."),
                Some("/assets/challenges/images/level-a2.svg"),
            ),
            level(
                "b1",
                "Level B1",
                Some("Telling stories: order events with nachdem, bevor and während."),
                Some("/assets/challenges/images/level-b1.svg"),
            ),
            level(
                "preview",
                "Preview",
                Some("Sample challenges: embedded and fetched datasets."),
                Some("/assets/challenges/images/level-preview.svg"),
            ),
        ]
    }

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
        (
            "Icons and Descriptions",
            SelectLevelCompProps {
                current: 1,
                levels: rich_levels(),
                on_select: Callback::noop(),
            }
        ),
        (
            "Icons Without Descriptions",
            SelectLevelCompProps {
                current: 0,
                levels: rich_levels()
                    .into_iter()
                    .map(|level| GamePath {
                        description: None,
                        ..level
                    })
                    .collect(),
                on_select: Callback::noop(),
            }
        ),
        (
            "Mixed",
            SelectLevelCompProps {
                current: 2,
                levels: vec![
                    level(
                        "a1",
                        "Level A1",
                        None,
                        Some("/assets/challenges/images/level-a1.svg"),
                    ),
                    level(
                        "a2",
                        "Level A2",
                        Some("Building sentences: join clauses with weil, dass and wenn."),
                        None,
                    ),
                    level("b1", "Level B1", None, None),
                ],
                on_select: Callback::noop(),
            }
        ),
    );
}
