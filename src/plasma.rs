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

/// Defines the available shape patterns for the plasma effect
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Shape {
    Ripple,
    Spiral,
    Circle,
    Square,
    Checkerboard,
}

/// Available color palettes for rendering the plasma effect
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Palette {
    Rainbow,
    BlueCyan,
    Hot,
    PurplePink,
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
}

impl Plasma {
    pub fn new(width: usize, height: usize, shape: Shape, palette: Palette) -> Self {
        Self {
            width,
            height,
            shape,
            palette,
        }
    }

    fn ripple(&self, dist: f32, time: f32) -> f32 {
        // Ripple pattern: sin(dist * 10.0 - time * 2.0)
        (dist * 10.0 - time * 2.0).sin()
    }

    fn spiral(&self, dist: f32, time: f32, angle: f32) -> f32 {
        // Spiral pattern: sin(dist * 10.0 + angle * 3.0 + time)
        (dist * 10.0 + angle * 3.0 + time).sin()
    }

    fn circle(&self, dist: f32, time: f32, angle: f32) -> f32 {
        // Circle pattern: sin(dist * 10.0 + time) + sin(angle * 2.0 + time)
        (dist * 10.0 + time).sin() + (angle * 2.0 + time).sin()
    }

    fn square(&self, px: f32, py: f32, min_dim: f32, time: f32) -> f32 {
        // Square pattern: sin(px / min_dim * 10.0 + time) * sin(py / min_dim * 10.0 + time)
        ((px / min_dim) * 10.0 + time).sin() * ((py / min_dim) * 10.0 + time).sin()
    }

    fn checkerboard(&self, px: f32, py: f32, min_dim: f32, time: f32) -> f32 {
        // Checkerboard pattern: sin(px / min_dim * 10.0) * sin(py / min_dim * 10.0)
        //                       + sin((px / min_dim + time) * 5.0) * sin((py / min_dim + time) * 5.0)
        ((px / min_dim) * 10.0).sin() * ((py / min_dim) * 10.0).sin()
            + ((px / min_dim + time) * 5.0).sin() * ((py / min_dim + time) * 5.0).sin()
    }

    pub fn draw(&self, buffer: &mut [u32], time: f32) {
        let w = self.width as f32;
        let h = self.height as f32;
        let center_x = w * 0.5;
        let center_y = h * 0.5;
        let min_dim = w.min(h) * 0.5;
        let alpha = 255 << 24;

        buffer
            .chunks_exact_mut(self.width)
            .enumerate()
            .for_each(|(y, row)| {
                let py = y as f32 - center_y;

                row.iter_mut().enumerate().for_each(|(x, pixel)| {
                    let px = x as f32 - center_x;
                    let dist = (px * px + py * py).sqrt() / min_dim;
                    let angle = py.atan2(px);

                    let v = match self.shape {
                        Shape::Ripple => self.ripple(dist, time),
                        Shape::Spiral => self.spiral(dist, time, angle),
                        Shape::Circle => self.circle(dist, time, angle),
                        Shape::Square => self.square(px, py, min_dim, time),
                        Shape::Checkerboard => self.checkerboard(px, py, min_dim, time),
                    };
                    let v = v * 0.5 + 0.5;

                    let (r, g, b) = match self.palette {
                        Palette::Rainbow => hsv_to_rgb(v * 360.0, 1.0, 1.0),
                        Palette::BlueCyan => hsv_to_rgb(v * 120.0 + 180.0, 0.8, 1.0),
                        Palette::Hot => hsv_to_rgb(v * 60.0, 1.0, 1.0),
                        Palette::PurplePink => hsv_to_rgb(v * 60.0 + 270.0, 0.7, 1.0),
                    };
                    *pixel = alpha | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                });
            });
    }
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
