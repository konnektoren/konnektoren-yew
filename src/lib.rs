pub mod app;
pub mod app_csr;
pub mod app_ssr;
pub mod components;
pub mod i18n;
pub mod managers;
pub mod model;
pub mod pages;
pub mod providers;
pub mod route;
pub mod switch_route;
pub mod tools;
pub mod wrapped_app;

#[cfg(feature = "ssg")]
pub mod ssg;

#[cfg(feature = "effects")]
pub mod effects;

#[cfg(feature = "storage")]
pub mod repository;

/// This is a prelude module that re-exports the most important types and traits.
pub mod prelude {
    pub use crate::app::App;
    #[cfg(feature = "csr")]
    pub use crate::app_csr::AppCSR;
    #[cfg(feature = "ssr")]
    pub use crate::app_ssr::AppSSR;
    pub use crate::components::*;
    #[cfg(feature = "effects")]
    pub use crate::effects::*;
    pub use crate::i18n::*;
    pub use crate::managers::*;
    pub use crate::model::*;
    pub use crate::providers::*;
    pub use crate::tools::{update_trace_from_response, TracedRequest, TracedResponse};
}
