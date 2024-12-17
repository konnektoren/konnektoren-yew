//! Profile components
#[cfg(feature = "storage")]
mod profile_config;

#[cfg(feature = "storage")]
mod profile_points;

#[cfg(feature = "storage")]
pub use profile_config::ProfileConfigComponent;

#[cfg(feature = "storage")]
pub use profile_points::ProfilePointsComponent;
