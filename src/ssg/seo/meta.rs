use crate::route::Route;
use yew::prelude::*;
use yew_router::Routable;

/// Metadata for a page
#[derive(Clone, Debug, PartialEq)]
pub struct PageMeta {
    /// Page title
    pub title: String,
    /// Page description
    pub description: String,
    /// Canonical URL path
    pub canonical_path: String,
    /// Open Graph type
    pub og_type: String,
    /// Language
    pub lang: String,
    /// Keywords
    pub keywords: Vec<String>,
    /// Image URL
    pub image_url: Option<String>,
}

impl Default for PageMeta {
    fn default() -> Self {
        Self {
            title: "Konnektoren".into(),
            description: "Konnektoren application".into(),
            canonical_path: "/".into(),
            og_type: "website".into(),
            lang: "en".into(),
            keywords: vec!["konnektoren".into()],
            image_url: None,
        }
    }
}

impl PageMeta {
    /// Create metadata for a specific route
    pub fn for_route(route: &Route) -> Self {
        let title = get_title_for_route(route);
        let description = get_description_for_route(route);
        let canonical_path = route.to_path();
        let keywords = generate_keywords_for_route(route);

        Self {
            title,
            description,
            canonical_path,
            keywords,
            ..Default::default()
        }
    }

    /// Generate HTML representation of the meta tags
    pub fn to_html(&self, base_url: &str) -> String {
        let mut meta = String::new();

        // Basic meta tags
        meta.push_str(&format!("<title>{}</title>\n", self.title));
        meta.push_str(&format!(
            "<meta name=\"description\" content=\"{}\" />\n",
            self.description
        ));
        meta.push_str(&format!(
            "<meta name=\"keywords\" content=\"{}\" />\n",
            self.keywords.join(", ")
        ));
        meta.push_str(&format!(
            "<meta http-equiv=\"content-language\" content=\"{}\" />\n",
            self.lang
        ));

        // Canonical URL
        let canonical_url = format!("{}{}", base_url, self.canonical_path);
        meta.push_str(&format!(
            "<link rel=\"canonical\" href=\"{}\" />\n",
            canonical_url
        ));

        // Open Graph meta tags
        meta.push_str(&format!(
            "<meta property=\"og:title\" content=\"{}\" />\n",
            self.title
        ));
        meta.push_str(&format!(
            "<meta property=\"og:description\" content=\"{}\" />\n",
            self.description
        ));
        meta.push_str(&format!(
            "<meta property=\"og:type\" content=\"{}\" />\n",
            self.og_type
        ));
        meta.push_str(&format!(
            "<meta property=\"og:url\" content=\"{}\" />\n",
            canonical_url
        ));

        // Add image if exists
        if let Some(image_url) = &self.image_url {
            meta.push_str(&format!(
                "<meta property=\"og:image\" content=\"{}\" />\n",
                image_url
            ));
        }

        // Twitter Card meta tags
        meta.push_str("<meta name=\"twitter:card\" content=\"summary\" />\n");
        meta.push_str(&format!(
            "<meta name=\"twitter:title\" content=\"{}\" />\n",
            self.title
        ));
        meta.push_str(&format!(
            "<meta name=\"twitter:description\" content=\"{}\" />\n",
            self.description
        ));

        // Add image if exists
        if let Some(image_url) = &self.image_url {
            meta.push_str(&format!(
                "<meta name=\"twitter:image\" content=\"{}\" />\n",
                image_url
            ));
        }

        meta
    }
}

/// Generate title for a specific route
pub fn get_title_for_route(route: &Route) -> String {
    match route {
        Route::Root => "Home".into(),
        Route::Home => "Home".into(),
        Route::About => "About".into(),
        Route::Example => "Example".into(),
    }
}

/// Generate description for a specific route
pub fn get_description_for_route(route: &Route) -> String {
    match route {
        Route::Root | Route::Home => "Welcome to Konnektoren - Connect and explore".into(),
        Route::About => "Learn more about Konnektoren and its features".into(),
        Route::Example => "Example page showcasing Konnektoren functionality".into(),
    }
}

/// Generate keywords for a specific route
pub fn generate_keywords_for_route(route: &Route) -> Vec<String> {
    let mut keywords = vec!["konnektoren".into()];

    match route {
        Route::Root | Route::Home => {
            keywords.extend(vec!["home".into(), "connect".into(), "explore".into()]);
        }
        Route::About => {
            keywords.extend(vec!["about".into(), "information".into(), "team".into()]);
        }
        Route::Example => {
            keywords.extend(vec!["example".into(), "demo".into(), "showcase".into()]);
        }
    }

    keywords
}

/// Component to render meta tags in Yew
#[derive(Properties, PartialEq)]
pub struct MetaTagsProps {
    pub meta: PageMeta,
    pub base_url: String,
}

#[function_component(MetaTags)]
pub fn meta_tags(props: &MetaTagsProps) -> Html {
    let meta = &props.meta;
    let base_url = &props.base_url;
    let canonical_url = format!("{}{}", base_url, meta.canonical_path);

    html! {
        <>
            <title>{ &meta.title }</title>
            <meta name="description" content={ meta.description.clone() } />
            <meta name="keywords" content={ meta.keywords.join(", ") } />
            <meta http-equiv="content-language" content={ meta.lang.clone() } />

            <link rel="canonical" href={ canonical_url.clone() } />

            <meta property="og:title" content={ meta.title.clone() } />
            <meta property="og:description" content={ meta.description.clone() } />
            <meta property="og:type" content={ meta.og_type.clone() } />
            <meta property="og:url" content={ canonical_url } />

            // Conditionally render image meta tags
            if let Some(image_url) = &meta.image_url {
                <meta property="og:image" content={ image_url.clone() } />
                <meta name="twitter:image" content={ image_url.clone() } />
            }

            <meta name="twitter:card" content="summary" />
            <meta name="twitter:title" content={ meta.title.clone() } />
            <meta name="twitter:description" content={ meta.description.clone() } />
        </>
    }
}

/// Helper function to generate a full HTML head section
pub fn generate_head_section(meta: &PageMeta, base_url: &str) -> String {
    let canonical_url = format!("{}{}", base_url, meta.canonical_path);

    format!(
        r#"<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <meta name="description" content="{}" />
    <meta name="keywords" content="{}" />
    <meta http-equiv="content-language" content="{}" />

    <link rel="canonical" href="{}" />

    <meta property="og:title" content="{}" />
    <meta property="og:description" content="{}" />
    <meta property="og:type" content="{}" />
    <meta property="og:url" content="{}" />
    {}

    <meta name="twitter:card" content="summary" />
    <meta name="twitter:title" content="{}" />
    <meta name="twitter:description" content="{}" />
    {}

    <link rel="stylesheet" href="/styles.css"/>
    <script defer src="/konnektoren-yew-bin.js"></script>
</head>"#,
        meta.title,
        meta.description,
        meta.keywords.join(", "),
        meta.lang,
        canonical_url,
        meta.title,
        meta.description,
        meta.og_type,
        canonical_url,
        meta.image_url.as_ref().map_or(String::new(), |url| format!(
            "<meta property=\"og:image\" content=\"{}\" />",
            url
        )),
        meta.title,
        meta.description,
        meta.image_url.as_ref().map_or(String::new(), |url| format!(
            "<meta name=\"twitter:image\" content=\"{}\" />",
            url
        ))
    )
}
