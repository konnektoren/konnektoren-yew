use axum::response::Html as AxumHtml;
use yew_router::Routable;

/// Generate HTML for a page with title and metadata
pub async fn render_html(
    content: String,
    title: &str,
    description: Option<&str>,
    route_path: &str,
    base_url: &str,
) -> AxumHtml<String> {
    let desc = description.unwrap_or(title);

    AxumHtml(format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{} - Konnektoren</title>
        <meta name="description" content="{}" />
        <!-- Canonical URL helps prevent duplicate content issues -->
        <link rel="canonical" href="{}{}" />
        <!-- Open Graph meta tags for social sharing -->
        <meta property="og:title" content="{} - Konnektoren" />
        <meta property="og:description" content="{}" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="{}{}" />
        <!-- Include the main CSS file that Trunk generates -->
        <link rel="stylesheet" href="/styles.css"/>
        <!-- Include your app's JavaScript bundle -->
        <script defer src="/konnektoren-yew-bin.js"></script>
    </head>
    <body>
        <div id="app" data-route="{}">{}</div>
    </body>
</html>"#,
        title, desc, base_url, route_path, title, desc, base_url, route_path, route_path, content
    ))
}

/// Generate HTML without Axum response wrapper (for file writing)
pub fn render_html_string(
    content: String,
    title: &str,
    description: Option<&str>,
    route_path: &str,
    base_url: &str,
) -> String {
    let desc = description.unwrap_or(title);

    format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{} - Konnektoren</title>
        <meta name="description" content="{}" />
        <!-- Canonical URL helps prevent duplicate content issues -->
        <link rel="canonical" href="{}{}" />
        <!-- Open Graph meta tags for social sharing -->
        <meta property="og:title" content="{} - Konnektoren" />
        <meta property="og:description" content="{}" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="{}{}" />
        <!-- Include the main CSS file that Trunk generates -->
        <link rel="stylesheet" href="/styles.css"/>
        <!-- Include your app's JavaScript bundle -->
        <script defer src="/konnektoren-yew-bin.js"></script>
    </head>
    <body>
        <div id="app" data-route="{}">{}</div>
    </body>
</html>"#,
        title, desc, base_url, route_path, title, desc, base_url, route_path, route_path, content
    )
}

/// Generate a standard page title from a route
pub fn get_title_for_route<R: Routable + std::fmt::Debug>(route: &R) -> String {
    // Default implementation based on route name
    // You can customize this for specific routes
    format!("{:?}", route)
        .replace("Route::", "")
        .replace(':', "")
        .replace('{', "")
        .replace('}', "")
}

/// Generate a standard page description from a route
pub fn get_description_for_route<R: Routable + std::fmt::Debug>(route: &R) -> String {
    format!("Konnektoren - {}", get_title_for_route(route))
}

/// Build metadata for a specific route
pub struct PageMetadata {
    pub title: String,
    pub description: String,
    pub canonical_path: String,
}

/// Generate metadata for a route
pub fn metadata_for_route<R: Routable + std::fmt::Debug + Clone>(route: &R) -> PageMetadata {
    let title = get_title_for_route(route);
    let description = get_description_for_route(route);
    let canonical_path = match route.to_path() {
        path if path.is_empty() => "/".to_string(),
        path => path,
    };

    PageMetadata {
        title,
        description,
        canonical_path,
    }
}
