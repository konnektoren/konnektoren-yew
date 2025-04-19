use env_logger::{Builder, Env};
use konnektoren_yew::app_ssr::App;
use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew_router::Routable;
use yew_router::prelude::*;
use yew_ssg::generators::{MetaTagGenerator, OpenGraphGenerator};
use yew_ssg::{SsgConfigBuilder, StaticSiteGenerator};

const ENV_BASE_URL: &str = "BASE_URL";
const ENV_SITE_NAME: &str = "SITE_NAME";
const DEFAULT_SITE_NAME: &str = "Konnektoren";
const DEFAULT_KEYWORDS: &str = "konnektoren,rust,yew";

#[derive(Clone, Routable, PartialEq, Debug, EnumIter)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/examples")]
    Examples,
    #[at("/certificates")]
    Certificates,
    #[at("/marketplace")]
    Marketplace,
    #[at("/backup")]
    Backup,
    #[at("/settings")]
    Settings,
    #[at("/chat")]
    Chat,
    #[at("/tour")]
    Tour,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("🏗️ Configuring static site generator...");

    // Load configuration
    let base_url = env::var(ENV_BASE_URL).unwrap_or_else(|_| "https://localhost".to_string());
    let site_name = env::var(ENV_SITE_NAME).unwrap_or_else(|_| DEFAULT_SITE_NAME.to_string());

    // Configure SSG
    let mut builder = SsgConfigBuilder::new()
        .output_dir("dist")
        .template("dist/index.html");

    // Add global metadata
    let mut global_meta = HashMap::new();
    global_meta.insert("site_name".to_string(), site_name);
    global_meta.insert("base_url".to_string(), base_url.clone());
    builder = builder.global_metadata(global_meta);

    // Add route metadata
    builder = add_route_metadata(builder, &base_url);

    // Add generators
    builder = builder
        .add_generator(MetaTagGenerator {
            default_description: "Konnektoren application".to_string(),
            default_keywords: vec!["konnektoren".to_string(), "rust".to_string()],
        })
        .add_generator(OpenGraphGenerator {
            site_name: DEFAULT_SITE_NAME.to_string(),
            default_image: "".to_string(),
        });

    // Build and run generator
    let config = builder.build();
    let generator = StaticSiteGenerator::new(config)?;
    generator.generate::<Route, App>().await?;

    info!("✅ Static site generation complete!");
    Ok(())
}

fn add_route_metadata(builder: SsgConfigBuilder, base_url: &str) -> SsgConfigBuilder {
    let mut builder = builder;

    // Add metadata for each route
    for route in Route::iter() {
        let path = route.to_path();
        let mut route_meta = HashMap::new();

        // Set route-specific metadata
        match route {
            Route::Home => {
                route_meta.insert("title".to_string(), "Home | Konnektoren".to_string());
                route_meta.insert(
                    "description".to_string(),
                    "Welcome to Konnektoren".to_string(),
                );
            }
            // Add other routes as needed
            _ => {
                route_meta.insert("title".to_string(), format!("{:?} | Konnektoren", route));
                route_meta.insert(
                    "description".to_string(),
                    "Konnektoren application page".to_string(),
                );
            }
        }

        route_meta.insert("keywords".to_string(), DEFAULT_KEYWORDS.to_string());
        route_meta.insert("canonical".to_string(), format!("{}{}", base_url, path));

        builder = builder.route_metadata(&path, route_meta);
    }

    builder
}
