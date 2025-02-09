pub struct WindowDimensions {
    pub width: usize,
    pub height: usize,
}

pub struct DemoBase {
    pub dim: WindowDimensions,
}

impl DemoBase {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            dim: WindowDimensions { width, height },
        }
    }
}

pub trait DemoEffect {
    fn draw(&self, buffer: &mut [u32], time: f32);
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h % 360.0; // Hue angle normalized to [0, 360) degrees
    let c = v * s; // Chroma: color intensity based on saturation and value
    let h_prime = h / 60.0; // Hue sector (divides color wheel into 6 sectors of 60° each)
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs()); // Secondary chroma component for color mixing
    let m = v - c; // Lightness adjustment to match value parameter

    let (r, g, b) = match h_prime as u8 {
        0 => (c, x, 0.0), // Red to Yellow: R constant, G increasing
        1 => (x, c, 0.0), // Yellow to Green: R decreasing, G constant
        2 => (0.0, c, x), // Green to Cyan: G constant, B increasing
        3 => (0.0, x, c), // Cyan to Blue: G decreasing, B constant
        4 => (x, 0.0, c), // Blue to Magenta: B constant, R increasing
        5 => (c, 0.0, x), // Magenta to Red: R constant, B decreasing
        _ => (c, 0.0, x), // Fallback case (should not occur with normalized input)
    };

    (
        (r + m).mul_add(255.0, 0.5) as u8,
        (g + m).mul_add(255.0, 0.5) as u8,
        (b + m).mul_add(255.0, 0.5) as u8,
    )
}
