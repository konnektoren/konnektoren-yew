//! Loading `faqs.yml` at runtime through the asset loader instead of
//! embedding it into the binary.
//!
//! - CSR: fetched once from `/assets/<path>` ([`DefaultAssetSource`]) and
//!   cached in a shared [`AssetServer`], so several components on a page
//!   trigger a single request.
//! - SSR/SSG and native: read synchronously from the filesystem
//!   ([`FileSource`]: `$BUILD_DIR`, `./`, `assets/`), so pre-rendered pages
//!   contain the FAQs.

use konnektoren_core::assets::AssetFormat;
use konnektoren_rs::faq::FaqData;
use std::sync::Arc;
use yew::prelude::*;

/// Default asset path of the FAQ file.
pub const FAQ_ASSET_PATH: &str = "faqs.yml";

/// Parses `faqs.yml` into [`FaqData`].
#[derive(Debug, Clone, Copy, Default)]
pub struct FaqFormat;

impl AssetFormat for FaqFormat {
    type Asset = FaqData;
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

/// State of [`use_faq_data`].
#[derive(Debug, Clone, PartialEq)]
pub enum FaqDataState {
    Loading,
    Loaded(Arc<FaqData>),
    Failed(AttrValue),
}

impl FaqDataState {
    /// The loaded data, or empty data while loading or after an error.
    pub fn data(&self) -> Arc<FaqData> {
        match self {
            Self::Loaded(data) => data.clone(),
            Self::Loading | Self::Failed(_) => Arc::default(),
        }
    }

    pub fn is_loaded(&self) -> bool {
        matches!(self, Self::Loaded(_))
    }
}

/// Loads the FAQ file at `path` (relative to the asset root, e.g.
/// [`FAQ_ASSET_PATH`]).
#[hook]
pub fn use_faq_data(path: AttrValue) -> FaqDataState {
    #[cfg(feature = "csr")]
    {
        csr::use_faq_data(path)
    }
    #[cfg(not(feature = "csr"))]
    {
        let state = use_state(|| file::load(&path));
        (*state).clone()
    }
}

#[cfg(feature = "csr")]
mod csr {
    use super::*;
    use konnektoren_asset_loader::AssetServer;
    use konnektoren_core::assets::DefaultAssetSource;
    use std::rc::Rc;

    thread_local! {
        static SERVER: AssetServer<DefaultAssetSource> =
            AssetServer::new(Rc::new(DefaultAssetSource::default()));
    }

    #[hook]
    pub(super) fn use_faq_data(path: AttrValue) -> FaqDataState {
        let state = use_state(|| FaqDataState::Loading);
        {
            let state = state.clone();
            use_effect_with(path, move |path| {
                let path = path.to_string();
                let server = SERVER.with(Clone::clone);
                wasm_bindgen_futures::spawn_local(async move {
                    state.set(match server.load(FaqFormat, path).await {
                        Ok(data) => FaqDataState::Loaded(data),
                        Err(error) => FaqDataState::Failed(error.to_string().into()),
                    });
                });
            });
        }
        (*state).clone()
    }
}

#[cfg(not(feature = "csr"))]
mod file {
    use super::*;
    use konnektoren_core::assets::{AssetSource, FileSource};
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    /// `FileSource` reads with `std::fs` and never awaits, so one poll completes it.
    pub(super) fn load(path: &str) -> FaqDataState {
        let source = FileSource::default();
        let mut future = pin!(source.load(&FaqFormat, path));
        match future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
        {
            Poll::Ready(Ok(data)) => FaqDataState::Loaded(Arc::new(data)),
            Poll::Ready(Err(error)) => FaqDataState::Failed(error.to_string().into()),
            Poll::Pending => FaqDataState::Loading,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn loads_from_filesystem() {
            // Tests run in the crate directory; the sample lives in the repository's assets/.
            let state = load("../../assets/faqs.yml");
            assert!(state.is_loaded(), "{state:?}");
            assert!(!state.data().faqs.is_empty());
        }

        #[test]
        fn missing_file_fails() {
            assert!(matches!(load("missing-faqs.yml"), FaqDataState::Failed(_)));
        }
    }
}
