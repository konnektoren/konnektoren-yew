use super::{SeasonalEvent, VisibleTime};
use chrono::DateTime;

/// Black Friday 2026, November 25 through 28 in Europe/Berlin.
pub struct BlackFridayEvent;

impl SeasonalEvent for BlackFridayEvent {
    fn visible_time() -> VisibleTime {
        VisibleTime::Once {
            start_ms: DateTime::parse_from_rfc3339("2026-11-25T00:00:00+01:00")
                .expect("valid Black Friday start")
                .timestamp_millis(),
            end_ms: DateTime::parse_from_rfc3339("2026-11-29T00:00:00+01:00")
                .expect("valid Black Friday end")
                .timestamp_millis(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_friday_uses_berlin_midnight_boundaries() {
        let window = BlackFridayEvent::visible_time();
        assert!(!window.contains(1_795_561_199_999, 11, 24));
        assert!(window.contains(1_795_561_200_000, 11, 25));
        assert!(window.contains(1_795_906_799_999, 11, 28));
        assert!(!window.contains(1_795_906_800_000, 11, 29));
    }
}
