use crate::i18n::use_i18n;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavigationHeaderProps {
    pub brand: Html,
    #[prop_or_default]
    pub tools: Html,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub content_target: Option<AttrValue>,
}

/// Header with a brand, tool area, optional skip link, and navigation content.
#[function_component(NavigationHeader)]
pub fn navigation_header(props: &NavigationHeaderProps) -> Html {
    let i18n = use_i18n();
    html! {
        <header class="navigation-header">
            {props.content_target.as_ref().map(|target| html! {
                <a class="navigation-header__skip" href={format!("#{target}")}>{i18n.t("Skip to content")}</a>
            }).unwrap_or_default()}
            <div class="navigation-header__topbar">
                <div class="navigation-header__brand">{props.brand.clone()}</div>
                <div class="navigation-header__tools">{props.tools.clone()}</div>
            </div>
            {for props.children.iter()}
        </header>
    }
}
