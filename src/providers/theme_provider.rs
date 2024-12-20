use yew::prelude::*;

use crate::model::Theme;

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: UseStateHandle<Theme>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme = use_state(Theme::get_from_body);

    let context = ThemeContext { theme };

    html! {
        <ContextProvider<ThemeContext> context={context}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub fn use_theme() -> UseStateHandle<Theme> {
    use_context::<ThemeContext>()
        .expect("use_theme must be used within a ThemeProvider")
        .theme
}
