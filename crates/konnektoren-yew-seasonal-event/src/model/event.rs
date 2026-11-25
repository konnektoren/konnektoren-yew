use super::VisibleTime;

pub trait SeasonalEvent {
    fn visible_time() -> VisibleTime;
}
