//! This module contains all the components that are used in the app.
pub mod advertisement;
pub mod app_version;
mod badge;
pub mod buy_me_coffee;
#[cfg(feature = "certificates")]
mod certificates;
pub mod challenge;
pub mod challenge_config;
pub mod challenge_info;
pub mod challenge_presence;
pub mod challenge_rating;
pub mod challenge_review;
pub mod challenge_timer;
pub mod chat;
pub mod feedback_popup;
pub mod game_path;
#[cfg(feature = "gdrive")]
pub mod gdrive_backup;
pub mod inbox;
pub mod leaderboard;
pub mod logo;
mod map;
#[cfg(feature = "marketplace")]
pub mod marketplace;
#[cfg(feature = "music")]
pub mod music;
pub mod navigation;
pub mod profile;
pub mod progress_bar;
mod rating_stars;
pub mod seo;
mod settings;
pub mod share_page;
pub mod social_links;
pub mod status_message;
pub mod timer;
#[cfg(feature = "tour")]
pub mod tour;
pub mod translate;

pub use advertisement::{AdNetwork, AdvertisementComponent, AdvertisementProps};
pub use app_version::AppVersionComponent;
pub use badge::Badge;
pub use buy_me_coffee::BuyMeCoffeeComponent;
#[cfg(feature = "certificates")]
pub use certificates::*;
pub use challenge::*;
pub use challenge_config::ChallengeConfigComponent;
pub use challenge_info::ChallengeInfoComponent;
pub use challenge_presence::ChallengePresenceComponent;
pub use challenge_rating::ChallengeRatingComponent;
pub use challenge_review::ChallengeReviewComponent;
pub use challenge_timer::ChallengeTimerComponent;
pub use chat::ChatComponent;
pub use feedback_popup::FeedbackPopup;
pub use game_path::GamePathComponent;
#[cfg(feature = "gdrive")]
pub use gdrive_backup::GDriveBackupComponent;
pub use inbox::InboxComponent;
pub use leaderboard::LeaderboardComp;
pub use logo::Logo;
pub use map::*;
#[cfg(feature = "marketplace")]
pub use marketplace::*;
#[cfg(feature = "music")]
pub use music::MusicComponent;
pub use navigation::*;
#[cfg(feature = "storage")]
pub use profile::ProfileConfigComponent;
#[cfg(feature = "storage")]
pub use profile::ProfilePointsComponent;
pub use progress_bar::ProgressBar;
pub use rating_stars::RatingStarsComponent;
pub use seo::{SeoComponent, SeoConfig};
pub use settings::*;
pub use share_page::SharePageComp;
pub use social_links::SocialLinks;
pub use status_message::{StatusMessage, StatusType};
pub use timer::TimerComponent;
#[cfg(feature = "tour")]
pub use tour::*;
pub use translate::TranslateComponent;
