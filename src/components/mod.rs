//! This module contains all the components that are used in the app.

#[cfg(feature = "certificates")]
mod certificates;
pub mod challenge;
pub mod challenge_config;
pub mod challenge_info;
pub mod challenge_presence;
pub mod challenge_rating;
pub mod challenge_review;
pub mod challenge_timer;
pub mod game_path;
#[cfg(feature = "gdrive")]
pub mod gdrive_backup;
pub mod inbox;
mod map;
#[cfg(feature = "music")]
pub mod music;
pub mod navigation;
pub mod profile;
pub mod progress_bar;
pub mod seo;
mod settings;
pub mod share_page;
pub mod timer;
pub mod translate;

#[cfg(feature = "marketplace")]
pub mod marketplace;
mod rating_stars;

pub use challenge::*;
pub use challenge_config::ChallengeConfigComponent;
pub use challenge_info::ChallengeInfoComponent;
pub use challenge_presence::ChallengePresenceComponent;
pub use challenge_rating::ChallengeRatingComponent;
pub use challenge_review::ChallengeReviewComponent;
pub use challenge_timer::ChallengeTimerComponent;
pub use game_path::GamePathComponent;
#[cfg(feature = "gdrive")]
pub use gdrive_backup::GDriveBackupComponent;
pub use inbox::InboxComponent;
pub use map::*;
pub use navigation::*;
pub use progress_bar::ProgressBar;
pub use seo::{SeoComponent, SeoConfig};
pub use share_page::SharePageComp;
pub use timer::TimerComponent;
pub use translate::TranslateComponent;

#[cfg(feature = "certificates")]
pub use certificates::*;

#[cfg(feature = "storage")]
pub use profile::ProfileConfigComponent;
#[cfg(feature = "storage")]
pub use profile::ProfilePointsComponent;

#[cfg(feature = "music")]
pub use music::MusicComponent;

#[cfg(feature = "marketplace")]
pub use marketplace::*;

pub use rating_stars::RatingStarsComponent;
pub use settings::*;
