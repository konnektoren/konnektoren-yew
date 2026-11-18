use konnektoren_yew_i18n::{I18nConfig, I18nContext, SelectedLanguage};
use konnektoren_yew_navigation::NavigationHeader;
use yew::prelude::*;

#[function_component(TestApp)]
fn test_app() -> Html {
    let config = use_state(|| {
        let mut config = I18nConfig::default();
        config.translations.insert(
            "en".into(),
            serde_json::json!({"Skip to content": "Jump to content"}),
        );
        config
    });
    let context = I18nContext {
        config,
        selected_language: SelectedLanguage::new("en"),
    };
    html! {
        <ContextProvider<I18nContext> {context}>
            <NavigationHeader brand={html! {"Brand"}} content_target="main" classes={classes!("app-header")} />
        </ContextProvider<I18nContext>>
    }
}

#[tokio::test]
async fn navigation_uses_the_app_i18n_context() {
    let output = yew::ServerRenderer::<TestApp>::new().render().await;
    assert!(output.contains("Jump to content"));
    assert!(output.contains("navigation-header app-header"));
}

#[derive(Clone, PartialEq, Debug, yew_router::Routable)]
enum TestRoute {
    #[at("/")]
    Home,
}

#[function_component(ResponsiveTestApp)]
fn responsive_test_app() -> Html {
    let config = use_state(I18nConfig::default);
    let context = I18nContext {
        config,
        selected_language: SelectedLanguage::new("en"),
    };
    html! {
        <ContextProvider<I18nContext> {context}>
            <konnektoren_yew_navigation::ResponsiveNavigation<TestRoute>
                items={vec![]} brand={html! {"Brand"}} secondary_label="Others"
                content_target="main" bottom_classes={classes!("app-bottom")} />
        </ContextProvider<I18nContext>>
    }
}

#[tokio::test]
async fn responsive_navigation_renders_both_layouts_and_preserves_slots() {
    let output = yew::ServerRenderer::<ResponsiveTestApp>::new()
        .render()
        .await;
    assert!(output.contains("navigation-header"));
    assert!(output.contains("navigation-bar--inline"));
    assert!(output.contains("navigation-bar--bottom app-bottom"));
    assert!(output.contains("href=\"#main\""));
    assert!(output.contains("Brand"));
}
