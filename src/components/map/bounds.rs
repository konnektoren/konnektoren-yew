use crate::components::ModelCoordinate;
use konnektoren_core::game::GamePath;

pub trait Bounds {
    fn get_bounds(&self) -> (ModelCoordinate, ModelCoordinate);
}

impl Bounds for GamePath {
    fn get_bounds(&self) -> (ModelCoordinate, ModelCoordinate) {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;

        if let Some(map) = &self.map {
            x_min = 0;
            y_min = 0;
            x_max = map.width as i32;
            y_max = map.height as i32;
        }

        for challenge_config in self.challenges.iter() {
            if let Some((x, y)) = challenge_config.position {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        (ModelCoordinate(x_min, y_min), ModelCoordinate(x_max, y_max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::challenges::ChallengeConfig;
    use konnektoren_core::game::GamePath;

    #[test]
    fn test_get_bounds() {
        let game_path = GamePath {
            challenges: vec![
                ChallengeConfig {
                    id: "konnektoren-1".to_string(),
                    position: Some((0, 0)),
                    ..Default::default()
                },
                ChallengeConfig {
                    id: "konnektoren-2".to_string(),
                    position: Some((2, 2)),
                    ..Default::default()
                },
                ChallengeConfig {
                    id: "konnektoren-2".to_string(),
                    position: Some((1, 3)),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let (min, max) = game_path.get_bounds();
        assert_eq!(min, ModelCoordinate(0, 0));
        assert_eq!(max, ModelCoordinate(2, 3));
    }

    #[test]
    fn test_bounds_with_map() {
        let game_path = GamePath {
            map: Some(konnektoren_core::game::Map {
                background: "background".to_string(),
                width: 10,
                height: 10,
            }),
            challenges: vec![ChallengeConfig {
                position: Some((0, 0)),
                ..Default::default()
            }],
            ..Default::default()
        };

        let (min, max) = game_path.get_bounds();
        assert_eq!(min, ModelCoordinate(0, 0));
        assert_eq!(max, ModelCoordinate(10, 10));
    }
}
