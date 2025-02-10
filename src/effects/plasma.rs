use crate::common::{hsv_to_rgb, DemoBase, DemoEffect};
use clap::ValueEnum;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Shape {
    Ripple,
    Spiral,
    Circle,
    Square,
    Checkerboard,
}

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Palette {
    Rainbow,
    BlueCyan,
    Hot,
    PurplePink,
}

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
