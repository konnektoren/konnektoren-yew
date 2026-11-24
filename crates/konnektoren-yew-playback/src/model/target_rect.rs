/// A target's position and size in viewport pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TargetRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl TargetRect {
    /// Center in viewport pixels.
    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}
