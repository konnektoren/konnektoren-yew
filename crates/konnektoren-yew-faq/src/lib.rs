//! FAQ components for Konnektoren applications: list, search and page.
//!
//! Re-exported by `konnektoren-yew` as `konnektoren_yew::faq` with its `faq`
//! feature (on by default).
//!
//! - [`use_faq_data`] loads `faqs.yml` through the asset loader at runtime, so
//!   apps do not embed it into their wasm.
//! - [`LazyFaqSearch`] builds its fuzzy search index on the first search.
//! - [`FaqSelection`] selects FAQs by id or tag from a URL hash.
//!
//! Styles: `styles/crates/faq/faq.css` in the `konnektoren-yew` repository.
//! These are included by its `styles/index.css`.

mod asset;
mod components;
mod search;
mod selection;

pub use asset::{FAQ_ASSET_PATH, FaqDataState, FaqFormat, use_faq_data};
pub use components::{
    FaqItem, FaqItemProps, FaqList, FaqListProps, FaqLoader, FaqLoaderProps, FaqTag, FaqTagProps,
};
pub use konnektoren_rs::faq::{Faq, FaqData};
pub use search::{FaqSearch, LazyFaqSearch};
pub use selection::{FaqSelection, normalized_hash};
