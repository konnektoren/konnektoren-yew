use chrono::{DateTime, Utc};

/// `2026-11-22 14:30 UTC`.
pub fn format_time(time: DateTime<Utc>) -> String {
    time.format("%Y-%m-%d %H:%M UTC").to_string()
}

/// An ISO timestamp like `2026-11-22T14:30:00.000Z` as `2026-11-22 14:30`; anything
/// else unchanged.
pub fn display_timestamp(timestamp: &str) -> String {
    match timestamp.split_once('T') {
        Some((date, time)) if time.len() >= 5 && time.is_char_boundary(5) => {
            format!("{date} {}", &time[..5])
        }
        _ => timestamp.to_owned(),
    }
}

/// A signed whole number with a typographic minus: `+12`, `−8`, `+0`.
pub fn signed(value: f64) -> String {
    format!("{value:+.0}").replace('-', "−")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_timestamps_and_signs() {
        assert_eq!(
            display_timestamp("2026-11-22T14:30:00.000Z"),
            "2026-11-22 14:30"
        );
        assert_eq!(display_timestamp("yesterday"), "yesterday");
        assert_eq!(signed(12.0), "+12");
        assert_eq!(signed(-8.0), "−8");
        assert_eq!(signed(0.0), "+0");
    }
}
