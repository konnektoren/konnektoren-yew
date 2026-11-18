//! This module contains all the components that are used in the app.
#[cfg(feature = "achievements")]
mod achievements;
pub mod ads;
pub mod analytics;
#[cfg(feature = "sbom")]
pub mod app_dependencies;
pub mod app_version;
mod badge;
#[cfg(feature = "certificates")]
mod certificates;
pub mod challenge;
pub mod challenge_completion;
pub mod challenge_config;
pub mod challenge_history;
pub mod challenge_info;
pub mod challenge_presence;
#[cfg(feature = "review")]
pub mod challenge_rating;
#[cfg(feature = "review")]
pub mod challenge_review;
pub mod challenge_timer;
#[cfg(feature = "chat")]
pub mod chat;
pub mod domain_selector;
pub mod faq;
#[cfg(feature = "review")]
pub mod feedback_popup;
#[cfg(all(feature = "csr", feature = "yew-preview"))]
pub mod game_loader_preview;
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
#[cfg(feature = "navigation")]
pub mod navigation;
pub mod profile;
pub mod progress_bar;
#[cfg(feature = "review")]
mod review_client;
pub mod seo;
mod settings;
pub mod share_page;
#[cfg(feature = "slideshow")]
pub mod slideshow;
pub mod social_links;
pub mod status;
pub mod status_message;
pub mod swipe;
pub mod timer;
#[cfg(feature = "tour")]
pub mod tour;
pub mod translate;
pub mod video;

#[cfg(feature = "achievements")]
pub use achievements::*;
pub use ads::{
    AdNetwork, AdvertisementComponent, AdvertisementProps, BlogAdComponent, BuyMeCoffeeComponent,
    GumroadProductAdComponent, GumroadSubscribeComponent,
};
pub use analytics::*;
#[cfg(feature = "sbom")]
pub use app_dependencies::AppDependenciesComponent;
pub use app_version::AppVersionComponent;
pub use badge::Badge;
#[cfg(feature = "certificates")]
pub use certificates::*;
pub use challenge::*;
pub use challenge_completion::*;
pub use challenge_config::ChallengeConfigComponent;
pub use challenge_history::*;
pub use challenge_info::ChallengeInfoComponent;
pub use challenge_presence::ChallengePresenceComponent;
#[cfg(feature = "review")]
pub use challenge_rating::ChallengeRatingComponent;
#[cfg(feature = "review")]
pub use challenge_review::ChallengeReviewComponent;
pub use challenge_timer::ChallengeTimerComponent;
#[cfg(feature = "chat")]
pub use chat::ChatComponent;
pub use domain_selector::{DomainSelectorComponent, DomainSelectorProps, DomainSelectorStyle};
pub use faq::{
    FaqItem, FaqItemProps, FaqList, FaqListProps, FaqLoader, FaqLoaderProps, FaqTag, FaqTagProps,
};
#[cfg(feature = "review")]
pub use feedback_popup::FeedbackPopup;
pub use game_path::GamePathComponent;
#[cfg(feature = "gdrive")]
pub use gdrive_backup::GDriveBackupComponent;
pub use inbox::InboxComponent;
#[cfg(feature = "review")]
pub use konnektoren_yew_review::RatingStarsComponent;
pub use leaderboard::LeaderboardComp;
pub use logo::Logo;
pub use map::*;
#[cfg(feature = "marketplace")]
pub use marketplace::*;
#[cfg(feature = "music")]
pub use music::MusicComponent;
#[cfg(feature = "navigation")]
pub use navigation::*;
#[cfg(feature = "storage")]
pub use profile::ProfileConfigComponent;
#[cfg(feature = "storage")]
pub use profile::ProfilePointsComponent;
pub use progress_bar::ProgressBar;
pub use seo::{SeoComponent, SeoConfig};
pub use settings::*;
pub use share_page::SharePageComp;
#[cfg(feature = "slideshow")]
pub use slideshow::*;
pub use social_links::SocialLinks;
pub use status::*;
pub use swipe::{SwipeComponent, SwipeDirection};
pub use timer::TimerComponent;
#[cfg(feature = "tour")]
pub use tour::*;
pub use translate::TranslateComponent;
pub use video::VideoComponent;

#[cfg(all(feature = "csr", feature = "yew-preview"))]
pub use game_loader_preview::GameLoaderPreview;
