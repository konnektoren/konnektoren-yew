/// Delay before advancing, adjusted by playback speed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaybackTiming {
    pub delay_ms: u32,
    /// A multiplier: `1.0` is normal, `2.0` is twice as fast.
    pub speed: f64,
}

impl PlaybackTiming {
    /// Returns no duration for a non-positive or non-finite speed.
    pub fn duration_ms(self) -> Option<u32> {
        if !self.speed.is_finite() || self.speed <= 0.0 {
            return None;
        }
        let duration = f64::from(self.delay_ms) / self.speed;
        Some(duration.min(f64::from(i32::MAX)).ceil() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_duration_and_rejects_invalid_speed() {
        let timing = PlaybackTiming {
            delay_ms: 1000,
            speed: 2.0,
        };
        assert_eq!(timing.duration_ms(), Some(500));
        assert_eq!(
            PlaybackTiming {
                speed: 0.5,
                ..timing
            }
            .duration_ms(),
            Some(2000)
        );
        assert_eq!(
            PlaybackTiming {
                speed: 0.0,
                ..timing
            }
            .duration_ms(),
            None
        );
        assert_eq!(
            PlaybackTiming {
                speed: f64::NAN,
                ..timing
            }
            .duration_ms(),
            None
        );
        assert_eq!(
            PlaybackTiming {
                speed: f64::MIN_POSITIVE,
                ..timing
            }
            .duration_ms(),
            Some(i32::MAX as u32)
        );
    }
}
