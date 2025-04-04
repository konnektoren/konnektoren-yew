use crate::route::Route;
use axum::response::Response;
use strum::IntoEnumIterator;
use yew_router::Routable;

/// Generate a sitemap.xml file for all routes
pub fn generate_sitemap(base_url: &str) -> Response<String> {
    let sitemap = generate_sitemap_string(base_url);

    Response::builder()
        .header("Content-Type", "application/xml")
        .body(sitemap)
        .unwrap()
}

/// Generate sitemap.xml as a string (for file writing)
pub fn generate_sitemap_string(base_url: &str) -> String {
    let mut sitemap = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">"#,
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
        let route_str = route.to_path();

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

    sitemap
}

/// Generate a sitemap.xml for any Routable type
pub fn generate_generic_sitemap<R>(base_url: &str) -> String
where
    R: Routable + IntoEnumIterator + Clone,
{
    let mut sitemap = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">"#,
    );

    // Add all routes
    for route in R::iter() {
        let route_path = route.to_path();

        sitemap.push_str(&format!(
            r#"
  <url>
    <loc>{}{}</loc>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>"#,
            base_url, route_path
        ));
    }

    sitemap.push_str("\n</urlset>");

    sitemap
}

/// Sitemap entry with additional metadata
pub struct SitemapEntry {
    pub loc: String,
    pub changefreq: String,
    pub priority: f32,
    pub lastmod: Option<String>,
}

impl SitemapEntry {
    /// Create a new sitemap entry
    pub fn new(loc: &str) -> Self {
        Self {
            loc: loc.to_string(),
            changefreq: "weekly".to_string(),
            priority: 0.8,
            lastmod: None,
        }
    }

    /// Set change frequency
    pub fn changefreq(mut self, freq: &str) -> Self {
        self.changefreq = freq.to_string();
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: f32) -> Self {
        self.priority = priority.clamp(0.0, 1.0);
        self
    }

    /// Set last modified date (YYYY-MM-DD format)
    pub fn lastmod(mut self, date: &str) -> Self {
        self.lastmod = Some(date.to_string());
        self
    }

    /// Format entry as XML
    pub fn to_xml(&self) -> String {
        let mut xml = format!(
            r#"  <url>
    <loc>{}</loc>
    <changefreq>{}</changefreq>
    <priority>{}</priority>"#,
            self.loc, self.changefreq, self.priority
        );

        if let Some(lastmod) = &self.lastmod {
            xml.push_str(&format!("\n    <lastmod>{}</lastmod>", lastmod));
        }

        xml.push_str("\n  </url>");

        xml
    }
}

/// Create a custom sitemap with detailed entries
pub fn create_custom_sitemap(base_url: &str, entries: Vec<SitemapEntry>) -> String {
    let mut sitemap = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">"#,
    );

    // Add all entries
    for entry in entries {
        sitemap.push_str("\n");
        sitemap.push_str(&entry.to_xml());
    }

    sitemap.push_str("\n</urlset>");

    sitemap
}
