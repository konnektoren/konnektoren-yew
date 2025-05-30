use axum::{Router, response::Html as AxumHtml, routing::get};
use konnektoren_yew::components::{AppVersionComponent, Badge};
use konnektoren_yew::prelude::App;
use log::info;
use yew::ServerRenderer;
use yew::prelude::*;

// Wrapper for AppVersionComponent
#[function_component(AppVersionWrapper)]
fn app_version_wrapper() -> Html {
    html! {
        <AppVersionComponent show_details={true} />
    }
}

// Wrapper for Badge
#[function_component(BadgeWrapper)]
fn badge_wrapper() -> Html {
    html! {
        <Badge
            label="Example Badge"
            description="This is an example badge rendered by the server."
        />
    }
}

async fn render_html(content: String) -> AxumHtml<String> {
    AxumHtml(format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Konnektoren Components</title>
        <style>
        </style>
    </head>
    <body>
        <h1>Konnektoren Component Server</h1>
        <p>This page demonstrates server-side rendering of Konnektoren components.</p>
        <main>{}</main>
    </body>
</html>"#,
        content
    ))
}

async fn serve_app() -> AxumHtml<String> {
    let app_renderer = ServerRenderer::<App>::new();
    let content = app_renderer.render().await;

    render_html(content).await
}

async fn serve_app_version() -> AxumHtml<String> {
    let renderer = ServerRenderer::<AppVersionWrapper>::new();
    let content = renderer.render().await;

    render_html(content).await
}

async fn serve_badge() -> AxumHtml<String> {
    let renderer = ServerRenderer::<BadgeWrapper>::new();
    let content = renderer.render().await;

    render_html(content).await
}

async fn serve_home() -> AxumHtml<String> {
    let app_version_renderer = ServerRenderer::<AppVersionWrapper>::new();
    let app_version_content = app_version_renderer.render().await;

    let badge_renderer = ServerRenderer::<BadgeWrapper>::new();
    let badge_content = badge_renderer.render().await;

    let combined_content = format!(
        r#"<div class="component-list">
                    {}
                    {}
            </div>

            <div class="component-links">
                <h2>Individual Component Pages</h2>
                <ul>
                    <li><a href="/app">App Component</a></li>
                    <li><a href="/app-version">App Version Component</a></li>
                    <li><a href="/badge">Badge Component</a></li>
                </ul>
            </div>
        </div>"#,
        app_version_content, badge_content
    );

    render_html(combined_content).await
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_home))
        .route("/app", get(serve_app))
        .route("/app-version", get(serve_app_version))
        .route("/badge", get(serve_badge));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Component server running on http://localhost:3000");

    let _server = axum::serve(listener, app).await.unwrap();
}
