pub mod app;
pub mod components;
pub mod i18n;
pub mod managers;
pub mod model;
pub mod providers;

#[cfg(feature = "effects")]
pub mod effects;

#[cfg(feature = "storage")]
pub mod repository;

/// This is a prelude module that re-exports the most important types and traits.
pub mod prelude {
    pub use crate::app::App;
    pub use crate::components::*;
    #[cfg(feature = "effects")]
    pub use crate::effects::*;
    pub use crate::i18n::*;
    pub use crate::managers::*;
    pub use crate::model::*;
    pub use crate::providers::*;
}
