use axum::{extract::Path, response::Html as AxumHtml, routing::get, Router};
use konnektoren_yew::prelude::App;
use konnektoren_yew::route::Route;
use konnektoren_yew::switch_route::switch_route;
use log::{error, info};
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew::ServerRenderer;
use yew_router::Routable;

#[derive(Properties, PartialEq)]
struct DirectRenderProps {
    route: Route,
}

#[function_component(DirectRender)]
fn direct_render(props: &DirectRenderProps) -> Html {
    switch_route(props.route.clone())
}

// Function to generate HTML shell around the rendered content
async fn render_html(content: String, title: &str) -> AxumHtml<String> {
    AxumHtml(format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{} - Konnektoren</title>
        <meta name="description" content="Konnektoren application - {}" />
        <!-- Include the main CSS file that Trunk generates -->
        <link data-trunk rel="css" href="/styles.css"/>
        <script defer data-trunk rel="inline" src="/hydration.js"></script>
    </head>
    <body>
        <div id="app">{}</div>
    </body>
</html>"#,
        title, title, content
    ))
}

// Serve the whole app at a specific route
async fn serve_route(Path(route_path): Path<String>) -> AxumHtml<String> {
    // Convert path to Route enum
    // Format the path for Router
    let route_path = format!("/{}/", route_path);

    // Use the Routable trait to recognize the route
    let route = Route::recognize(&route_path).unwrap_or(Route::Root);

    // Title based on route
    let title = match &route {
        Route::Root => "Home",
        Route::Home => "Home",
        Route::About => "About",
        Route::Example => "Example",
    };

    // Renderer for the whole app with the specific route
    let renderer = ServerRenderer::<DirectRender>::with_props(move || DirectRenderProps {
        route: route.clone(),
    });

    let content = renderer.render().await;
    render_html(content, title).await
}

// Serve the root app
async fn serve_root() -> AxumHtml<String> {
    let renderer = ServerRenderer::<App>::new();

    let content = renderer.render().await;
    render_html(content, "Home").await
}

// Serve sitemap.xml for SEO
async fn serve_sitemap() -> axum::response::Response<String> {
    let base_url = "https://konnektoren.app"; // Replace with your actual domain

    let mut sitemap = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
    );

    // Add root URL
    sitemap.push_str(&format!(
        r#"
  <url>
    <loc>{}</loc>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>"#,
        base_url
    ));

    // Add all other routes
    for route in Route::iter() {
        // Skip root as it's already added
        if matches!(route, Route::Root) {
            continue;
        }

        // Convert route to string path
        let route_str: String = route.to_path();

        sitemap.push_str(&format!(
            r#"
  <url>
    <loc>{}{}</loc>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>"#,
            base_url, route_str
        ));
    }

    sitemap.push_str("\n</urlset>");

    axum::response::Response::builder()
        .header("Content-Type", "application/xml")
        .body(sitemap)
        .unwrap()
}

// Function to serve robots.txt
async fn serve_robots() -> axum::response::Response<String> {
    let robots_txt =
        format!("User-agent: *\nAllow: /\nSitemap: https://konnektoren.app/sitemap.xml");

    axum::response::Response::builder()
        .header("Content-Type", "text/plain")
        .body(robots_txt)
        .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_root))
        .route("/sitemap.xml", get(serve_sitemap))
        .route("/robots.txt", get(serve_robots))
        .route("/{route}/", get(serve_route)); // Dynamic route handling

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("SSR server running on http://localhost:3000");

    let server = axum::serve(listener, app);
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}
