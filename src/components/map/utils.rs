use crate::components::map::bounds::Bounds;
use crate::prelude::ModelCoordinate;
use konnektoren_core::game::GamePath;

pub trait Zoom {
    fn get_zoom(&self, max_width: u32, max_height: u32) -> f64;
}

impl Zoom for GamePath {
    fn get_zoom(&self, max_width: u32, max_height: u32) -> f64 {
        let (min, max) = self.get_bounds();
        let width = max.0 - min.0;
        let height = max.1 - min.1;
        let width_zoom = max_width as f64 / width as f64;
        let height_zoom = max_height as f64 / height as f64;
        width_zoom.min(height_zoom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::challenges::ChallengeConfig;
    use konnektoren_core::game::GamePath;

    #[test]
    fn test_get_zoom() {
        let game_path = GamePath {
            challenges: vec![
                ChallengeConfig {
                    id: "konnektoren-1".to_string(),
                    position: Some((0, 0)),
                    ..Default::default()
                },
                ChallengeConfig {
                    id: "konnektoren-2".to_string(),
                    position: Some((20, 2)),
                    ..Default::default()
                },
                ChallengeConfig {
                    id: "konnektoren-2".to_string(),
                    position: Some((1, 20)),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(game_path.get_zoom(20, 20), 1.0);
    }
}
