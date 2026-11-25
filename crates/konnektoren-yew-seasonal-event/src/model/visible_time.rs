/// Half-open windows. Annual dates use one-based months and local time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisibleTime {
    Once { start_ms: i64, end_ms: i64 },
    EveryYear { start: (u32, u32), end: (u32, u32) },
}

impl VisibleTime {
    pub fn contains(&self, timestamp_ms: i64, month: u32, day: u32) -> bool {
        match self {
            Self::Once { start_ms, end_ms } => (*start_ms..*end_ms).contains(&timestamp_ms),
            Self::EveryYear { start, end } => {
                let date = (month, day);
                if start <= end {
                    date >= *start && date < *end
                } else {
                    date >= *start || date < *end
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VisibleTime;

    #[test]
    fn fixed_window_includes_start_and_excludes_end() {
        let window = VisibleTime::Once {
            start_ms: 100,
            end_ms: 200,
        };
        assert!(!window.contains(99, 1, 1));
        assert!(window.contains(100, 1, 1));
        assert!(window.contains(199, 1, 1));
        assert!(!window.contains(200, 1, 1));
    }

    #[test]
    fn annual_window_wraps_at_new_year() {
        let window = VisibleTime::EveryYear {
            start: (11, 21),
            end: (1, 1),
        };
        assert!(!window.contains(0, 11, 20));
        assert!(window.contains(0, 11, 21));
        assert!(window.contains(0, 12, 31));
        assert!(!window.contains(0, 1, 1));
    }
}
