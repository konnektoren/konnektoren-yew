// Represents a coordinate in the model (usually between 0 and 20)
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ModelCoordinate(pub i32, pub i32);

// Represents a coordinate in the SVG (scaled by SCALE factor)
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct SvgCoordinate(pub i32, pub i32);

// Represents a coordinate in the web browser (pixel values)
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct BrowserCoordinate(pub f64, pub f64);

impl ModelCoordinate {
    // Convert from ModelCoordinate to SvgCoordinate
    pub fn to_svg(self, scale: i32) -> SvgCoordinate {
        SvgCoordinate(self.0 * scale, self.1 * scale)
    }
}

impl SvgCoordinate {
    // Convert from SvgCoordinate to BrowserCoordinate
    pub fn to_browser(self) -> BrowserCoordinate {
        BrowserCoordinate(self.0 as f64, self.1 as f64)
    }
}

impl BrowserCoordinate {
    // Convert from BrowserCoordinate to ModelCoordinate (reverse of to_svg)
    pub fn to_model(self, scale: i32) -> ModelCoordinate {
        ModelCoordinate(
            (self.0 / scale as f64) as i32,
            (self.1 / scale as f64) as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_to_svg() {
        let model = ModelCoordinate(1, 2);
        let svg = model.to_svg(10);
        assert_eq!(svg, SvgCoordinate(10, 20));
    }

    #[test]
    fn test_svg_to_browser() {
        let svg = SvgCoordinate(10, 20);
        let browser = svg.to_browser();
        assert_eq!(browser, BrowserCoordinate(10.0, 20.0));
    }

    #[test]
    fn test_browser_to_model() {
        let browser = BrowserCoordinate(10.0, 20.0);
        let model = browser.to_model(10);
        assert_eq!(model, ModelCoordinate(1, 2));
    }
}
