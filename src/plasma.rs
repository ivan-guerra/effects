//! A module for generating animated plasma effects with various patterns and color palettes.
//!
//! This module provides functionality to create smooth, colorful plasma animations using
//! different geometric patterns (ripples, spirals, circles, etc.) and color schemes.
//! The plasma effect is generated by combining mathematical functions with color
//! transformations to create fluid, psychedelic patterns.
//!
//! # Example
//! ```
//! use plasma::{Plasma, Shape, Palette};
//!
//! let plasma = Plasma::new(800, 600, Shape::Ripple, Palette::Rainbow);
//! let mut buffer = vec![0u32; 800 * 600];
//! plasma.draw(&mut buffer, 0.0);
//! ```
use clap::ValueEnum;

/// Scale factor change for increasing or decreasing the plasma pattern density
const SCALE_DELTA: f32 = 10.0;

/// Defines the available shape patterns for the plasma effect
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Shape {
    Ripple,
    Spiral,
    Circle,
    Square,
}

/// Available color palettes for rendering the plasma effect
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Palette {
    Rainbow,
    BlueCyan,
    Hot,
    PurplePink,
    BlackWhite,
}

/// A plasma effect generator that creates colorful animated patterns
pub struct Plasma {
    /// Width of the plasma effect in pixels
    width: usize,
    /// Height of the plasma effect in pixels  
    height: usize,
    /// The geometric shape used to generate the plasma pattern
    shape: Shape,
    /// Color palette used for rendering the plasma effect
    palette: Palette,
    /// Scale factor that controls the density/size of the plasma patterns
    scale: f32,
}

impl Plasma {
    pub fn new(width: usize, height: usize, shape: Shape, palette: Palette, scale: f32) -> Self {
        Self {
            width,
            height,
            shape,
            palette,
            scale,
        }
    }

    /// Increases the scale factor of the plasma patterns by SCALE_DELTA.
    pub fn increase_scale(&mut self) {
        self.scale += SCALE_DELTA;
    }

    /// Decreases the scale factor of the plasma patterns by SCALE_DELTA.
    pub fn decrease_scale(&mut self) {
        self.scale -= SCALE_DELTA;
    }

    /// Cycles to the next color palette in the sequence.
    pub fn next_palette(&mut self) {
        self.palette = match self.palette {
            Palette::Rainbow => Palette::BlueCyan,
            Palette::BlueCyan => Palette::Hot,
            Palette::Hot => Palette::PurplePink,
            Palette::PurplePink => Palette::BlackWhite,
            Palette::BlackWhite => Palette::Rainbow,
        };
    }

    /// Cycles to the next shape pattern in the sequence.
    pub fn next_shape(&mut self) {
        self.shape = match self.shape {
            Shape::Ripple => Shape::Spiral,
            Shape::Spiral => Shape::Circle,
            Shape::Circle => Shape::Square,
            Shape::Square => Shape::Ripple,
        };
    }

    /// Cycles to the previous shape pattern in the sequence.
    pub fn prev_shape(&mut self) {
        self.shape = match self.shape {
            Shape::Ripple => Shape::Square,
            Shape::Spiral => Shape::Ripple,
            Shape::Circle => Shape::Spiral,
            Shape::Square => Shape::Circle,
        };
    }

    fn ripple(&self, dist: f32, time: f32) -> f32 {
        // Ripple pattern: sin(dist * 10.0 - time * 2.0)
        (dist * self.scale - time * 2.0).sin()
    }

    fn spiral(&self, dist: f32, time: f32, angle: f32) -> f32 {
        // Spiral pattern: sin(dist * 10.0 + angle * 3.0 + time)
        (dist * self.scale + angle * 3.0 + time).sin()
    }

    fn circle(&self, dist: f32, time: f32, angle: f32) -> f32 {
        // Circle pattern: sin(dist * 10.0 + time) + sin(angle * 2.0 + time)
        (dist * self.scale + time).sin() + (angle * 2.0 + time).sin()
    }

    fn square(&self, px: f32, py: f32, min_dim: f32, time: f32) -> f32 {
        // Square pattern: sin(px / min_dim * 10.0 + time) * sin(py / min_dim * 10.0 + time)
        ((px / min_dim) * self.scale + time).sin() * ((py / min_dim) * self.scale + time).sin()
    }

    /// Converts HSV (Hue, Saturation, Value) color values to RGB (Red, Green, Blue)
    ///
    /// # Arguments
    ///
    /// * `h` - Hue angle in degrees [0, 360)
    /// * `s` - Saturation value [0, 1]
    /// * `v` - Value/brightness [0, 1]
    ///
    /// # Returns
    ///
    /// A tuple of (red, green, blue) values as 8-bit unsigned integers [0, 255]
    fn hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        // Normalize hue to [0,360) degree range
        let h = ((h % 360.0) + 360.0) % 360.0;
        // Calculate chroma (color intensity) from value and saturation
        let c = v * s;
        // Convert hue to sector position (60° per sector)
        let h_prime = h / 60.0;
        // Calculate intermediate value for RGB conversion based on hue position
        let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
        // Calculate value adjustment to maintain brightness level
        let m = v - c;

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

    /// Renders the plasma effect into the provided pixel buffer.
    ///
    /// # Arguments
    /// * `buffer` - Mutable slice of u32 values representing the pixel buffer
    /// * `time` - Current time value in seconds, used for animation
    ///
    /// Each pixel in the buffer is updated with a color value based on the current
    /// shape, palette, and time parameters. The color values are packed into 32-bit
    /// ARGB format.
    pub fn draw(&self, buffer: &mut [u32], time: f32) {
        let w = self.width as f32;
        let h = self.height as f32;
        // Calculate the center coordinates of the display area
        let center_x = w * 0.5;
        let center_y = h * 0.5;
        // Calculate half of the smallest dimension for scaling patterns
        let min_dim = w.min(h) * 0.5;
        // Create alpha channel mask for ARGB color format (fully opaque)
        let alpha = 255 << 24;

        buffer
            .chunks_exact_mut(self.width)
            .enumerate()
            .for_each(|(y, row)| {
                // Calculate the y-coordinate relative to the center of the display
                let py = y as f32 - center_y;

                row.iter_mut().enumerate().for_each(|(x, pixel)| {
                    // Calculate the x-coordinate relative to the center of the display
                    let px = x as f32 - center_x;
                    // Calculate the normalized distance from the center point
                    let dist = (px * px + py * py).sqrt() / min_dim;
                    // Calculate the angle in radians from the center point
                    let angle = py.atan2(px);

                    let v = match self.shape {
                        Shape::Ripple => self.ripple(dist, time),
                        Shape::Spiral => self.spiral(dist, time, angle),
                        Shape::Circle => self.circle(dist, time, angle),
                        Shape::Square => self.square(px, py, min_dim, time),
                    };
                    // Normalize the plasma value from [-1,1] to [0,1] range for color mapping
                    let v = v * 0.5 + 0.5;

                    let (r, g, b) = match self.palette {
                        Palette::Rainbow => self.hsv_to_rgb(v * 360.0, 1.0, 1.0),
                        Palette::BlueCyan => self.hsv_to_rgb(v * 120.0 + 180.0, 0.8, 1.0),
                        Palette::Hot => self.hsv_to_rgb(v * 60.0, 1.0, 1.0),
                        Palette::PurplePink => self.hsv_to_rgb(v * 60.0 + 270.0, 0.7, 1.0),
                        Palette::BlackWhite => {
                            let gray = (v * 255.0) as u8;
                            (gray, gray, gray)
                        }
                    };
                    *pixel = alpha | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_plasma() -> Plasma {
        Plasma::new(800, 600, Shape::Ripple, Palette::Rainbow, 0.0)
    }

    #[test]
    fn scale_increases_by_scale_delta_when_increased() {
        let mut plasma = create_plasma();
        let initial_scale = plasma.scale;
        plasma.increase_scale();
        assert_eq!(plasma.scale, initial_scale + SCALE_DELTA);
    }

    #[test]
    fn scale_decreases_by_scale_delta_when_decreased() {
        let mut plasma = create_plasma();
        let initial_scale = plasma.scale;
        plasma.decrease_scale();
        assert_eq!(plasma.scale, initial_scale - SCALE_DELTA);
    }

    #[test]
    fn shape_cycles_forward_through_all_variants() {
        let mut plasma = create_plasma();

        assert_eq!(plasma.shape, Shape::Ripple);
        plasma.next_shape();
        assert_eq!(plasma.shape, Shape::Spiral);
        plasma.next_shape();
        assert_eq!(plasma.shape, Shape::Circle);
        plasma.next_shape();
        assert_eq!(plasma.shape, Shape::Square);
        plasma.next_shape();
        assert_eq!(plasma.shape, Shape::Ripple);
    }

    #[test]
    fn shape_cycles_backward_through_all_variants() {
        let mut plasma = create_plasma();

        assert_eq!(plasma.shape, Shape::Ripple);
        plasma.prev_shape();
        assert_eq!(plasma.shape, Shape::Square);
        plasma.prev_shape();
        assert_eq!(plasma.shape, Shape::Circle);
        plasma.prev_shape();
        assert_eq!(plasma.shape, Shape::Spiral);
        plasma.prev_shape();
        assert_eq!(plasma.shape, Shape::Ripple);
    }

    #[test]
    fn shape_returns_to_initial_after_complete_cycle_in_both_directions() {
        let mut plasma = create_plasma();
        let initial_shape = plasma.shape.clone();

        // Do a full cycle with next_shape
        for _ in 0..4 {
            plasma.next_shape();
        }
        assert_eq!(
            plasma.shape, initial_shape,
            "Shape should return to initial after full next cycle"
        );

        // Do a full cycle with prev_shape
        for _ in 0..4 {
            plasma.prev_shape();
        }
        assert_eq!(
            plasma.shape, initial_shape,
            "Shape should return to initial after full prev cycle"
        );
    }

    #[test]
    fn palette_cycles_through_all_variants() {
        let mut plasma = create_plasma();

        assert_eq!(plasma.palette, Palette::Rainbow);
        plasma.next_palette();
        assert_eq!(plasma.palette, Palette::BlueCyan);
        plasma.next_palette();
        assert_eq!(plasma.palette, Palette::Hot);
        plasma.next_palette();
        assert_eq!(plasma.palette, Palette::PurplePink);
        plasma.next_palette();
        assert_eq!(plasma.palette, Palette::BlackWhite);
        plasma.next_palette();
        assert_eq!(plasma.palette, Palette::Rainbow);
    }

    #[test]
    fn palette_returns_to_initial_after_complete_cycle() {
        let mut plasma = create_plasma();
        let initial_palette = plasma.palette.clone();

        // Do a full cycle
        for _ in 0..5 {
            plasma.next_palette();
        }

        assert_eq!(
            plasma.palette, initial_palette,
            "Palette should return to initial after full cycle"
        );
    }

    #[test]
    fn each_palette_transition_changes_to_different_variant() {
        let mut plasma = create_plasma();
        let mut previous_palette = plasma.palette.clone();

        for _ in 0..5 {
            plasma.next_palette();
            assert_ne!(
                plasma.palette, previous_palette,
                "Each palette transition should result in a different variant"
            );
            previous_palette = plasma.palette.clone();
        }
    }

    #[test]
    fn hsv_to_rgb_converts_primary_colors_correctly() {
        let plasma = create_plasma();

        // Red (0° hue)
        let (r, g, b) = plasma.hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!((r, g, b), (255, 0, 0), "Pure red should be (255, 0, 0)");

        // Green (120° hue)
        let (r, g, b) = plasma.hsv_to_rgb(120.0, 1.0, 1.0);
        assert_eq!((r, g, b), (0, 255, 0), "Pure green should be (0, 255, 0)");

        // Blue (240° hue)
        let (r, g, b) = plasma.hsv_to_rgb(240.0, 1.0, 1.0);
        assert_eq!((r, g, b), (0, 0, 255), "Pure blue should be (0, 0, 0)");
    }

    #[test]
    fn hsv_to_rgb_converts_secondary_colors_correctly() {
        let plasma = create_plasma();

        // Yellow (60° hue)
        let (r, g, b) = plasma.hsv_to_rgb(60.0, 1.0, 1.0);
        assert_eq!((r, g, b), (255, 255, 0), "Yellow should be (255, 255, 0)");

        // Cyan (180° hue)
        let (r, g, b) = plasma.hsv_to_rgb(180.0, 1.0, 1.0);
        assert_eq!((r, g, b), (0, 255, 255), "Cyan should be (0, 255, 255)");

        // Magenta (300° hue)
        let (r, g, b) = plasma.hsv_to_rgb(300.0, 1.0, 1.0);
        assert_eq!((r, g, b), (255, 0, 255), "Magenta should be (255, 0, 255)");
    }

    #[test]
    fn hsv_to_rgb_handles_grayscale_correctly() {
        let plasma = create_plasma();

        // Black (V = 0)
        let (r, g, b) = plasma.hsv_to_rgb(0.0, 0.0, 0.0);
        assert_eq!((r, g, b), (0, 0, 0), "Black should be (0, 0, 0)");

        // White (V = 1, S = 0)
        let (r, g, b) = plasma.hsv_to_rgb(0.0, 0.0, 1.0);
        assert_eq!(
            (r, g, b),
            (255, 255, 255),
            "White should be (255, 255, 255)"
        );

        // 50% Gray (V = 0.5, S = 0)
        let (r, g, b) = plasma.hsv_to_rgb(0.0, 0.0, 0.5);
        assert_eq!(
            (r, g, b),
            (128, 128, 128),
            "50% gray should be (128, 128, 128)"
        );
    }

    #[test]
    fn hsv_to_rgb_handles_hue_wrapping() {
        let plasma = create_plasma();

        // Test that 360° wraps to 0°
        let color1 = plasma.hsv_to_rgb(0.0, 1.0, 1.0);
        let color2 = plasma.hsv_to_rgb(360.0, 1.0, 1.0);
        assert_eq!(color1, color2, "0° and 360° hue should produce same color");

        // Test that negative hues work correctly
        let color3 = plasma.hsv_to_rgb(-120.0, 1.0, 1.0);
        let color4 = plasma.hsv_to_rgb(240.0, 1.0, 1.0);
        assert_eq!(
            color3, color4,
            "-120° and 240° hue should produce same color"
        );
    }

    #[test]
    fn hsv_to_rgb_handles_saturation_correctly() {
        let plasma = create_plasma();
        let hue = 0.0; // Red
        let value = 1.0;

        // Full saturation
        let (r1, g1, b1) = plasma.hsv_to_rgb(hue, 1.0, value);
        assert_eq!((r1, g1, b1), (255, 0, 0), "Full saturation red");

        // Half saturation
        let (r2, g2, b2) = plasma.hsv_to_rgb(hue, 0.5, value);
        assert_eq!((r2, g2, b2), (255, 128, 128), "Half saturation red");

        // Zero saturation (should be white at full value)
        let (r3, g3, b3) = plasma.hsv_to_rgb(hue, 0.0, value);
        assert_eq!(
            (r3, g3, b3),
            (255, 255, 255),
            "Zero saturation at full value"
        );
    }
}
