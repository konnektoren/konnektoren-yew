use super::{SeasonalEvent, VisibleTime};

/// The seasonal Advent period, using the visitor's local calendar.
pub struct AdventEvent;

impl SeasonalEvent for AdventEvent {
    fn visible_time() -> VisibleTime {
        VisibleTime::EveryYear {
            start: (11, 21),
            end: (1, 1),
        }
    }
}
