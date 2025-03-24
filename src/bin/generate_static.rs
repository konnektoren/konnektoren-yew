use konnektoren_yew::route::Route;
use konnektoren_yew::ssg::direct_renderer::{DirectRender, DirectRenderProps};
use konnektoren_yew::ssg::renderer::{metadata_for_route, render_html_string};
use konnektoren_yew::ssg::seo::meta::PageMeta;
use konnektoren_yew::ssg::seo::robots::generate_robots_txt_string;
use konnektoren_yew::ssg::seo::sitemap::generate_sitemap_string;
use konnektoren_yew::wrapped_app::{WrappedApp, WrappedAppProps};
use log::{error, info};
use std::fs;
use std::path::Path;
use std::process::{Child, Command};
use std::time::Duration;
use strum::IntoEnumIterator;
use tokio::time::sleep;
use yew::ServerRenderer;
use yew_router::Routable;

const BASE_URL: &str = "https://konnektoren.app";
const OUTPUT_DIR: &str = "static_dist";

/// Start SSR server for rendering
fn start_ssr_server() -> Result<Child, Box<dyn std::error::Error>> {
    info!("Starting SSR server...");

    let server = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "konnektoren-yew-server",
            "--features",
            "server",
        ])
        .spawn()?;

    Ok(server)
}

/// Copy assets from Trunk build
async fn copy_assets(source_dir: &str, dest_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(source_dir).exists() {
        error!(
            "{} directory not found. Run 'trunk build' first.",
            source_dir
        );
        return Err(format!("Missing {} directory", source_dir).into());
    }

    // Copy JS, WASM, CSS files
    let entries = fs::read_dir(source_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".js")
                || filename.ends_with(".wasm")
                || filename.ends_with(".css")
            {
                let dest_path = format!("{}/{}", dest_dir, filename);
                fs::copy(&path, dest_path)?;
                info!("Copied asset: {}", filename);
            }
        }
    }

    Ok(())
}

/// Render a component and save it to file
async fn render_and_save_route(
    route: Route,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clone route for different uses
    let route_for_renderer = route.clone();
    let route_for_meta = route.clone();
    let route_for_path = route.clone();

    // Create the renderer for this route
    let renderer = ServerRenderer::<WrappedApp>::with_props(move || WrappedAppProps {
        route: route_for_renderer.clone(),
    });

    // Render the HTML content
    let content = renderer.render().await;

    // Get metadata for the route
    let meta = PageMeta::for_route(&route_for_meta);

    // Determine file path
    let route_path = route_for_path.to_path();
    let (dir_path, file_path) = if route_path == "/" {
        (output_dir.to_string(), format!("{}/index.html", output_dir))
    } else {
        let path_component = route_path.trim_start_matches('/').trim_end_matches('/');
        let dir = format!("{}/{}", output_dir, path_component);
        (dir.clone(), format!("{}/index.html", dir))
    };

    // Create directory if it doesn't exist
    fs::create_dir_all(&dir_path)?;

    // Generate HTML and save to file
    let html = render_html_string(
        content,
        &meta.title,
        Some(&meta.description),
        &route_path,
        BASE_URL,
    );

    fs::write(file_path, html)?;

    info!("Generated: {}", route_path);
    Ok(())
}

/// Generate SEO files (sitemap.xml, robots.txt)
fn generate_seo_files(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Generate sitemap.xml
    let sitemap = generate_sitemap_string(BASE_URL);
    fs::write(format!("{}/sitemap.xml", output_dir), sitemap)?;
    info!("Generated: sitemap.xml");

    // Generate robots.txt
    let robots = generate_robots_txt_string(BASE_URL);
    fs::write(format!("{}/robots.txt", output_dir), robots)?;
    info!("Generated: robots.txt");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting static site generation...");

    // Check if trunk build exists
    if !Path::new("dist").exists() {
        error!("Trunk build directory not found. Run 'trunk build' first.");
        return Err("Missing Trunk build assets".into());
    }

    // Create output directory
    fs::create_dir_all(OUTPUT_DIR)?;

    // Option 1: Use SSR server (commenting this out as we'll use direct rendering)
    /*
    // Start SSR server
    let mut server = start_ssr_server()?;

    // Wait for server to start
    info!("Waiting for SSR server to start...");
    sleep(Duration::from_secs(3)).await;

    // Fetch pages via HTTP
    // ...

    // Shutdown server
    info!("Shutting down SSR server...");
    server.kill()?;
    */

    // Option 2: Direct rendering
    info!("Rendering routes directly...");

    // Generate pages for all routes
    for route in Route::iter() {
        render_and_save_route(route, OUTPUT_DIR).await?;
    }

    // Generate SEO files
    generate_seo_files(OUTPUT_DIR)?;

    // Copy assets from Trunk build
    info!("Copying assets from Trunk build...");
    copy_assets("dist", OUTPUT_DIR).await?;

    info!(
        "✅ Static site generation complete! Output directory: {}",
        OUTPUT_DIR
    );
    Ok(())
}
