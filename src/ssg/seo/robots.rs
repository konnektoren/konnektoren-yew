use axum::response::Response;

/// Generate robots.txt content for a website
pub fn generate_robots_txt(base_url: &str) -> Response<String> {
    let robots_txt = generate_robots_txt_string(base_url);

    Response::builder()
        .header("Content-Type", "text/plain")
        .body(robots_txt)
        .unwrap()
}

/// Generate robots.txt as a string (for file writing)
pub fn generate_robots_txt_string(base_url: &str) -> String {
    format!(
        r#"User-agent: *
Allow: /

# Sitemap location
Sitemap: {}/sitemap.xml

# Disallow specific directories (uncomment if needed)
# Disallow: /admin/
# Disallow: /private/"#,
        base_url
    )
}
