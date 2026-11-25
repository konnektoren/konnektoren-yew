use super::{SeasonalEvent, VisibleTime};
use chrono::DateTime;

/// Black Friday 2026, from midnight to midnight in Europe/Berlin.
pub struct BlackFridayEvent;

impl SeasonalEvent for BlackFridayEvent {
    fn visible_time() -> VisibleTime {
        VisibleTime::Once {
            start_ms: DateTime::parse_from_rfc3339("2026-11-27T00:00:00+01:00")
                .expect("valid Black Friday start")
                .timestamp_millis(),
            end_ms: DateTime::parse_from_rfc3339("2026-11-28T00:00:00+01:00")
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
        assert!(!window.contains(1_795_733_999_999, 11, 26));
        assert!(window.contains(1_795_734_000_000, 11, 27));
        assert!(window.contains(1_795_820_399_999, 11, 27));
        assert!(!window.contains(1_795_820_400_000, 11, 28));
    }
}
