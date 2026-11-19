//! YAML credits catalogs. Text fields are translation keys; names can also be plain text.
use konnektoren_asset_loader::AssetFormat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CreditsCatalog {
    pub groups: Vec<CreditGroup>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditGroup {
    pub id: String,
    pub name: String,
    pub credits: Vec<CreditEntry>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditEntry {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub links: Vec<CreditLink>,
    #[serde(default)]
    pub licenses: Vec<LicenseFile>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditLink {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseFile {
    pub name: String,
    /// Path relative to the configured asset source, not the manifest's directory.
    pub file: String,
}

/// A shared asset server for a credits catalog and its license files.
#[derive(Clone)]
pub(crate) struct CreditAssetServer(
    pub(crate)  std::rc::Rc<
        konnektoren_asset_loader::AssetServer<dyn konnektoren_asset_loader::SourceLayer>,
    >,
);

impl PartialEq for CreditAssetServer {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Default)]
pub struct CreditsFormat;

impl AssetFormat for CreditsFormat {
    type Asset = CreditsCatalog;
    type Error = serde_yaml::Error;
    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }
    fn cache_key(&self) -> Option<String> {
        Some(String::new())
    }
    fn parse(&self, bytes: &[u8]) -> Result<Self::Asset, Self::Error> {
        serde_yaml::from_slice(bytes)
    }
}

#[derive(Default)]
pub struct LicenseTextFormat;

impl AssetFormat for LicenseTextFormat {
    type Asset = String;
    type Error = std::string::FromUtf8Error;
    fn extensions(&self) -> &[&str] {
        &["txt", "md"]
    }
    fn cache_key(&self) -> Option<String> {
        Some(String::new())
    }
    fn parse(&self, bytes: &[u8]) -> Result<Self::Asset, Self::Error> {
        String::from_utf8(bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_asset_loader::{AssetError, AssetServer, AssetSource};
    use std::{cell::RefCell, rc::Rc};

    #[derive(Default)]
    struct RecordingSource(RefCell<Vec<String>>);

    impl AssetSource for RecordingSource {
        async fn load_bytes(&self, path: &str) -> Result<Vec<u8>, AssetError> {
            self.0.borrow_mut().push(path.into());
            match path {
                "credits.yml" => Ok(include_bytes!("../../assets/credits.example.yml").to_vec()),
                "licenses/example.txt" => Ok(b"<example license & plain text>".to_vec()),
                _ => Err(AssetError::NotFound { path: path.into() }),
            }
        }
    }

    #[tokio::test]
    async fn manifest_does_not_prefetch_licenses_and_text_is_cached() {
        let source = Rc::new(RecordingSource::default());
        let server = AssetServer::new(source.clone());
        let catalog = server.load(CreditsFormat, "credits.yml").await.unwrap();
        assert_eq!(*source.0.borrow(), ["credits.yml"]);
        let file = &catalog.groups[0].credits[0].licenses[0].file;
        let first = server.load(LicenseTextFormat, file).await.unwrap();
        let second = server.load(LicenseTextFormat, file).await.unwrap();
        assert!(std::sync::Arc::ptr_eq(&first, &second));
        assert_eq!(first.as_str(), "<example license & plain text>");
        assert_eq!(*source.0.borrow(), ["credits.yml", "licenses/example.txt"]);
    }

    #[test]
    fn invalid_manifests_and_non_utf8_text_fail_to_parse() {
        assert!(CreditsFormat.parse(b"groups: invalid").is_err());
        assert!(LicenseTextFormat.parse(&[0xff]).is_err());
    }

    #[test]
    fn vendor_catalog_covers_the_stylesheet_imports() {
        let catalog = CreditsFormat
            .parse(include_bytes!("../../assets/vendor-credits.yml"))
            .unwrap();
        let credits = &catalog.groups[0].credits;
        for name in [
            "Inter",
            "JetBrains Mono",
            "Font Awesome Free 6.5.2",
            "Intro.js 7.2.0",
        ] {
            assert!(
                credits
                    .iter()
                    .any(|credit| credit.name == name && !credit.links.is_empty())
            );
        }
    }
}
