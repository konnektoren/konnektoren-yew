//! Stable Yew handle for sharing the platform game loader and its asset cache.
use konnektoren_asset_loader::{AssetServer, AssetSource, SourceLayer};
use konnektoren_platform::{game_loader::GameLoader, manifest::ManifestConfig};
use std::rc::Rc;

#[derive(Clone)]
pub struct GameLoaderHandle(Rc<GameLoader<dyn SourceLayer>>);

impl GameLoaderHandle {
    pub fn from_manifest(
        manifest: &impl ManifestConfig,
        source: impl AssetSource + 'static,
    ) -> Self {
        let source: Rc<dyn SourceLayer> = Rc::new(source);
        Self::with_server(manifest, AssetServer::new(source))
    }

    pub fn with_server(
        manifest: &impl ManifestConfig,
        server: AssetServer<dyn SourceLayer>,
    ) -> Self {
        Self::new(GameLoader::with_server(manifest, server))
    }

    pub fn new(loader: GameLoader<dyn SourceLayer>) -> Self {
        Self(Rc::new(loader))
    }

    pub fn loader(&self) -> &GameLoader<dyn SourceLayer> {
        &self.0
    }
}

impl PartialEq for GameLoaderHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[cfg(feature = "yew-preview")]
pub(crate) mod preview {
    use super::*;
    use konnektoren_asset_loader::{AssetError, LayeredSource, UrlSource};
    use konnektoren_core::assets::DEFAULT_ASSETS;
    use konnektoren_platform::manifest::{Manifest, NoExtensions};

    // Core defaults have flat keys; the platform loader uses challenges/... keys.
    struct CoreDefaults;
    impl AssetSource for CoreDefaults {
        async fn load_bytes(&self, path: &str) -> Result<Vec<u8>, AssetError> {
            DEFAULT_ASSETS
                .load_bytes(path.strip_prefix("challenges/").unwrap_or(path))
                .await
        }
    }

    pub(crate) fn challenge_path(id: &str) -> String {
        match id {
            "multiple-choice" => "challenges/multiple_choice.yml".to_owned(),
            _ => format!("challenges/{id}.yml"),
        }
    }

    pub(crate) fn loader() -> Result<GameLoaderHandle, String> {
        let manifest: Manifest<NoExtensions> =
            serde_yaml::from_str(include_str!("../../assets/konnektoren.manifest.yml"))
                .map_err(|error| error.to_string())?;
        let source = LayeredSource::default()
            .with_source(CoreDefaults)
            .with_source(UrlSource::new(format!(
                "/{}",
                manifest.asset_path().trim_matches('/')
            )));
        let source: Rc<dyn SourceLayer> = Rc::new(source);
        let loader = GameLoader::with_server(&manifest, AssetServer::new(source))
            .with_challenge_path_resolver(challenge_path);
        Ok(GameLoaderHandle::new(loader))
    }
}
