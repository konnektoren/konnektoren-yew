/// A target's position and size in viewport pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TargetRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
