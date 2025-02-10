//! A classic plasma demo effect with various shape patterns and color palettes
//!
//! This module implements a configurable plasma effect that can render different
//! patterns including ripples, spirals, circles, squares, and checkerboards.
//! Each pattern can be displayed using different color palettes such as rainbow,
//! hot colors, or cool colors.
//!
//! # Example
//!
//! ```
//! use demo_effects::effects::plasma::{Config, Shape, Palette};
//!
//! let plasma = Config::new(
//!     800,    // width
//!     600,    // height
//!     Shape::Ripple,
//!     Palette::Rainbow
//! );
//!
//! plasma.draw(&mut buffer, time);
//! ```
use crate::common::{hsv_to_rgb, DemoBase, DemoEffect};
use clap::ValueEnum;

/// Defines the available shape patterns for the plasma effect
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Shape {
    /// Creates concentric wave patterns that ripple outward from the center,
    /// producing a water-like effect
    Ripple,
    /// Generates rotating spiral patterns that create a hypnotic swirling effect
    Spiral,
    /// Renders circular patterns that expand and contract from multiple center points
    Circle,
    /// Produces geometric patterns based on square shapes with hard edges
    Square,
    /// Creates an alternating pattern of squares resembling a chess board
    Checkerboard,
}

/// Available color palettes for rendering the plasma effect
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Palette {
    /// Full spectrum color cycling through all hues of the rainbow
    Rainbow,
    /// Cool colors focusing on blues and cyans, creating a calm, oceanic feel
    BlueCyan,
    /// Warm colors using reds, oranges and yellows, resembling fire or heat
    Hot,
    /// Vibrant colors cycling through purples, magentas and pinks
    PurplePink,
}

/// Configuration for the plasma effect
pub struct Config {
    base: DemoBase,
    plasma_type: Shape,
    palette: Palette,
}

impl Config {
    pub fn new(width: usize, height: usize, plasma_type: Shape, palette: Palette) -> Self {
        Self {
            base: DemoBase::new(width, height),
            plasma_type,
            palette,
        }
    }
}

impl DemoEffect for Config {
    fn draw(&self, buffer: &mut [u32], time: f32) {
        let w = self.base.dim.width as f32;
        let h = self.base.dim.height as f32;
        let center_x = w * 0.5;
        let center_y = h * 0.5;
        let min_dim = w.min(h) * 0.5;
        let alpha = 255 << 24;

        buffer
            .chunks_exact_mut(self.base.dim.width)
            .enumerate()
            .for_each(|(y, row)| {
                let py = y as f32 - center_y;

                row.iter_mut().enumerate().for_each(|(x, pixel)| {
                    let px = x as f32 - center_x;
                    let dist = (px * px + py * py).sqrt() / min_dim;
                    let angle = py.atan2(px);

                    let v = match self.plasma_type {
                        Shape::Ripple => (dist * 10.0 - time * 2.0).sin(),
                        Shape::Spiral => (dist * 10.0 + angle * 3.0 + time).sin(),
                        Shape::Circle => (dist * 10.0 + time).sin() + (angle * 2.0 + time).sin(),
                        Shape::Square => {
                            ((px / min_dim) * 10.0 + time).sin()
                                * ((py / min_dim) * 10.0 + time).sin()
                        }
                        Shape::Checkerboard => {
                            ((px / min_dim) * 10.0).sin() * ((py / min_dim) * 10.0).sin()
                                + ((px / min_dim + time) * 5.0).sin()
                                    * ((py / min_dim + time) * 5.0).sin()
                        }
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
