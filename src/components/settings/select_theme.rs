use crate::model::Theme;
use crate::providers::use_theme;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectThemeProps {
    #[prop_or_default]
    pub themes: Option<Vec<Theme>>,
}

#[function_component(SelectTheme)]
pub fn select_theme(props: &SelectThemeProps) -> Html {
    let theme = use_theme();

    let update_theme = |new_theme: &Theme| {
        if let Some(html) = gloo::utils::document().document_element() {
            html.set_attribute(
                "data-theme",
                match new_theme {
                    Theme::Light => "light",
                    Theme::Dark => "dark",
                    Theme::Star => "cyberpunk",
                    Theme::Other(name) => name.as_str(),
                },
            )
            .expect("Failed to set theme");
        }
    };

    {
        let theme = theme.clone();
        use_effect(move || {
            update_theme(&theme);
            || ()
        });
    }

    let available_themes = props
        .themes
        .clone()
        .unwrap_or_else(|| vec![Theme::Light, Theme::Dark, Theme::Star]);

    let get_next_theme = move |current_theme: Theme| -> Theme {
        let current_index = available_themes
            .iter()
            .position(|t| t == &current_theme)
            .unwrap_or(0);

        let next_index = (current_index + 1) % available_themes.len();
        available_themes[next_index].clone()
    };

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = get_next_theme((*theme).clone());
            theme.set(new_theme);
        })
    };

    let get_theme_icon = |theme: &Theme| -> &str {
        match theme {
            Theme::Light => "fa-sun",
            Theme::Dark => "fa-moon",
            Theme::Star => "fa-star",
            Theme::Other(_) => "fa-circle",
        }
    };

    let get_theme_name = |theme: &Theme| -> String {
        match theme {
            Theme::Light => "Light Theme".to_string(),
            Theme::Dark => "Dark Theme".to_string(),
            Theme::Star => "Star Theme".to_string(),
            Theme::Other(name) => name.to_string(),
        }
    };

    html! {
        <div class="select-theme">
            <button onclick={toggle_theme}>
                <i class={classes!("fas", get_theme_icon(&theme))}></i>
                <span>{get_theme_name(&theme)}</span>
            </button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SelectTheme,
        SelectThemeProps {
            themes: Some(vec![
                Theme::Light,
                Theme::Dark,
                Theme::Star,
                Theme::Other("custom".to_string())
            ])
        },
        (
            "Light / Dark Theme",
            SelectThemeProps {
                themes: Some(vec![Theme::Light, Theme::Dark])
            }
        )
    );
}
