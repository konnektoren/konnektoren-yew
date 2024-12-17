//! This module contains the map component and its subcomponents.
mod bounds;
mod coordinates;
mod map;
mod svg_challenge;
mod svg_map;
mod svg_path;
mod utils;

pub type ChallengeIndex = usize;
const SCALE: i32 = 10;

pub use coordinates::{BrowserCoordinate, ModelCoordinate, SvgCoordinate};
pub use map::MapComponent;
