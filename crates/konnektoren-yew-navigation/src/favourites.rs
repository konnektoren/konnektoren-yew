use super::NavItem;
use konnektoren_yew_i18n::use_i18n;
use yew::prelude::*;

/// A controlled favourites editor; the application owns selection and persistence.
#[derive(Properties, PartialEq)]
pub struct NavigationFavouritesProps<Route: Clone + PartialEq + 'static> {
    pub items: Vec<(String, NavItem<Route>)>,
    pub selected: Vec<String>,
    pub on_toggle: Callback<String>,
    pub on_reorder: Callback<(usize, usize)>,
    #[prop_or_default]
    pub fixed_items: Vec<NavItem<Route>>,
    #[prop_or(4)]
    pub max: usize,
    #[prop_or_else(|| "Choose favourites for your navigation.".into())]
    pub description: AttrValue,
    #[prop_or_else(|| "Others".into())]
    pub secondary_label: AttrValue,
}

#[function_component(NavigationFavourites)]
pub fn navigation_favourites<Route: Clone + PartialEq + 'static>(
    props: &NavigationFavouritesProps<Route>,
) -> Html {
    let i18n = use_i18n();
    html! {
        <div class="navigation-favourites">
            <p>{i18n.t(&props.description)}</p>
            <div class="navigation-favourites__preview" aria-label={i18n.t("Navigation preview")}>
                <p class="navigation-favourites__preview-title">{i18n.t("Navigation preview")}</p>
                <div class="navigation-favourites__queue">
                    {for props.fixed_items.iter().filter_map(|item| {
                        let NavItem::Route { name, icon, .. } = item else { return None };
                        Some(html! { <span class="navigation-favourites__item"><span class="navigation-favourites__icon" aria-hidden="true">{icon.clone()}</span>{i18n.t(name)}</span> })
                    })}
                    {for props.selected.iter().enumerate().filter_map(|(index, id)| {
                        let (_, NavItem::Route { name, icon, .. }) = props.items.iter().find(|(key, _)| key == id)? else { return None };
                        let earlier = props.on_reorder.clone();
                        let later = props.on_reorder.clone();
                        Some(html! {
                            <span class="navigation-favourites__item">
                                <span class="navigation-favourites__icon" aria-hidden="true">{icon.clone()}</span><span>{i18n.t(name)}</span>
                                <button type="button" class="navigation-favourites__move" aria-label={format!("{}: {}", i18n.t("Move earlier"), i18n.t(name))} disabled={index == 0} onclick={Callback::from(move |_| earlier.emit((index, index.saturating_sub(1))))}>{"←"}</button>
                                <button type="button" class="navigation-favourites__move" aria-label={format!("{}: {}", i18n.t("Move later"), i18n.t(name))} disabled={index + 1 == props.selected.len()} onclick={Callback::from(move |_| later.emit((index, index + 1)))}>{"→"}</button>
                            </span>
                        })
                    })}
                    <span class="navigation-favourites__item">{i18n.t(&props.secondary_label)}</span>
                </div>
                <p>{i18n.t("Use the arrows to change the order of your favourites.")}</p>
            </div>
            <div class="navigation-favourites__choices">
                {for props.items.iter().filter_map(|(id, item)| {
                    let NavItem::Route { name, icon, .. } = item else { return None };
                    let checked = props.selected.contains(id);
                    let disabled = !checked && props.selected.len() >= props.max;
                    let toggle = props.on_toggle.clone();
                    let id = id.clone();
                    Some(html! {
                        <label class="navigation-favourites__choice">
                            <input type="checkbox" class="checkbox checkbox-primary" {checked} {disabled} onchange={Callback::from(move |_| toggle.emit(id.clone()))} />
                            <span class="navigation-favourites__icon" aria-hidden="true">{icon.clone()}</span>
                            <span>{i18n.t(name)}</span>
                        </label>
                    })
                })}
            </div>
            <p aria-live="polite">{format!("{} / {}", props.selected.len(), props.max)}</p>
        </div>
    }
}
