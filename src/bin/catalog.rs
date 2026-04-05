//! Generate a static HTML component catalog for konnektoren-yew.
//!
//! Renders every SSR-safe component variant via Yew SSR and writes a single
//! self-contained HTML file — a long-scroll catalog of all components.
//!
//! Run with:
//!   cargo run --bin catalog --features catalog
//!
//! Override the output path via the OUTPUT environment variable:
//!   OUTPUT=dist/catalog.html cargo run --bin catalog --features catalog

use konnektoren_yew::app::preview_groups;
use konnektoren_yew::model::DefaultSessionInitializer;
use konnektoren_yew::prelude::create_i18n_config;
use konnektoren_yew::providers::create_repositories;
use konnektoren_yew::repository::LocalStorage;
use std::sync::Arc;
use yew::prelude::*;

#[cfg(feature = "yew-preview")]
use konnektoren_yew::prelude::{
    DesignProvider, GameControllerProvider, I18nProvider, RepositoryProvider, ThemeProvider,
};

fn main() {
    let output = std::env::var("OUTPUT").ok().map(std::path::PathBuf::from);

    let css_file = std::env::var("CSS_FILE").ok().map(std::path::PathBuf::from);

    let mut options = yew_preview::CatalogOptions::new("konnektoren-yew");

    if let Some(out) = output {
        options = options.output(out);
    }

    if let Some(css) = css_file {
        options = options.css_file(css);
    }

    options = options.wrapper(|node| {
        let i18n_config = create_i18n_config();
        let storage = LocalStorage::new(None);
        let session_initializer = DefaultSessionInitializer;
        let repository_config = create_repositories(storage, Arc::new(session_initializer));

        html! {
            <RepositoryProvider config={repository_config}>
            <ThemeProvider>
            <DesignProvider>
            <I18nProvider config={i18n_config}>
                <GameControllerProvider>
                    {node}
                </GameControllerProvider>
            </I18nProvider>
            </DesignProvider>
            </ThemeProvider>
            </RepositoryProvider>
        }
    });

    yew_preview::generate_catalog_blocking(preview_groups(), options);
}
