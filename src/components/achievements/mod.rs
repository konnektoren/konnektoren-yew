mod achievement;
#[cfg(feature = "certificates")]
mod achievements;

pub use achievement::AchievementComponent;
#[cfg(feature = "certificates")]
pub use achievements::AchievementsComponent;
