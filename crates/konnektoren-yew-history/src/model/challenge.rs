//! Challenge attempts as history records, judged on the app's
//! [`PerformanceScale`] from `konnektoren-core`: a filter by [`PerformanceBand`] and
//! sort orders.
use super::{
    Direction, FilterContext, HistoryFilter, HistoryItem, HistoryOption, HistorySort, SortEntry,
    SortKey, Tone,
};
use crate::components::History;
use chrono::{DateTime, Utc};
use konnektoren_core::challenges::{Challenge, Performance, PerformanceBand, PerformanceScale};
use yew::AttrValue;

/// A history of challenge attempts. Set [`SCALE`](Self::SCALE) to the app's scale so
/// the [`Results`] filter and [`challenge_result`] judge like the rest of the app.
///
/// ```ignore
/// impl ChallengeHistoryPage for ExerciseHistory {
///     const SCALE: PerformanceScale = crate::SCALE;
/// }
/// ```
pub trait ChallengeHistoryPage: History<Item = Challenge> {
    const SCALE: PerformanceScale = PerformanceScale::DEFAULT;
}

impl HistoryItem for Challenge {
    /// The configured challenge id; attempts of one challenge share it.
    fn key(&self) -> AttrValue {
        self.challenge_config.id.clone().into()
    }

    fn time(&self) -> Option<DateTime<Utc>> {
        self.end_time.or(self.start_time)
    }

    /// The configured name, else the name of the exercise type.
    fn name(&self) -> AttrValue {
        if self.challenge_config.name.trim().is_empty() {
            self.challenge_type.name().to_string().into()
        } else {
            self.challenge_config.name.clone().into()
        }
    }
}

/// The attempt's result in percent.
pub fn challenge_score(challenge: &Challenge) -> u32 {
    challenge.performance(&challenge.challenge_result)
}

/// The colour of a band: mastered good, good informative, passed neutral, not passed a
/// warning.
pub fn band_tone(band: PerformanceBand) -> Tone {
    match band {
        PerformanceBand::Mastered => Tone::Good,
        PerformanceBand::Good => Tone::Info,
        PerformanceBand::Passed => Tone::Neutral,
        PerformanceBand::NotPassed => Tone::Warn,
    }
}

/// Translation key of a band.
pub fn band_label(band: PerformanceBand) -> &'static str {
    match band {
        PerformanceBand::Mastered => "Mastered",
        PerformanceBand::Good => "Good",
        PerformanceBand::Passed => "Passed",
        PerformanceBand::NotPassed => "Not passed",
    }
}

/// Filter by the band of the result on the page's scale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Results {
    #[default]
    All,
    Mastered,
    Good,
    Passed,
    NotPassed,
}

impl Results {
    /// The band this choice lists; `None` for all.
    pub fn band(self) -> Option<PerformanceBand> {
        match self {
            Self::All => None,
            Self::Mastered => Some(PerformanceBand::Mastered),
            Self::Good => Some(PerformanceBand::Good),
            Self::Passed => Some(PerformanceBand::Passed),
            Self::NotPassed => Some(PerformanceBand::NotPassed),
        }
    }
}

impl HistoryOption for Results {
    const ALL: &'static [Self] = &[
        Self::All,
        Self::Mastered,
        Self::Good,
        Self::Passed,
        Self::NotPassed,
    ];

    fn value(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Mastered => "mastered",
            Self::Good => "good",
            Self::Passed => "passed",
            Self::NotPassed => "not-passed",
        }
    }

    fn label(self) -> &'static str {
        self.band().map_or("All results", band_label)
    }
}

impl<H: ChallengeHistoryPage> HistoryFilter<H> for Results {
    const ID: &'static str = "result";
    const LABEL: &'static str = "Result";

    fn admits(self, challenge: &Challenge, _: &FilterContext<'_, H>) -> bool {
        self.band()
            .is_none_or(|band| H::SCALE.band(challenge_score(challenge)) == band)
    }
}

/// Orders of attempts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChallengeSort {
    #[default]
    Newest,
    Oldest,
    Best,
    Lowest,
    Name,
}

impl HistoryOption for ChallengeSort {
    const ALL: &'static [Self] = &[
        Self::Newest,
        Self::Oldest,
        Self::Best,
        Self::Lowest,
        Self::Name,
    ];

    fn value(self) -> &'static str {
        match self {
            Self::Newest => "newest",
            Self::Oldest => "oldest",
            Self::Best => "best",
            Self::Lowest => "lowest",
            Self::Name => "name",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Newest => "Newest first",
            Self::Oldest => "Oldest first",
            Self::Best => "Best result first",
            Self::Lowest => "Lowest result first",
            Self::Name => "Exercise name",
        }
    }
}

impl<H: History<Item = Challenge>> HistorySort<H> for ChallengeSort {
    fn key(self, entry: &SortEntry<'_, Challenge>) -> SortKey {
        match self {
            Self::Newest | Self::Oldest => SortKey::Base,
            Self::Best | Self::Lowest => SortKey::Number(Some(challenge_score(entry.item) as f64)),
            Self::Name => SortKey::Text(entry.name.to_owned().into()),
        }
    }

    fn direction(self) -> Direction {
        match self {
            Self::Oldest | Self::Lowest | Self::Name => Direction::Ascending,
            Self::Newest | Self::Best => Direction::Descending,
        }
    }
}
