//! A command-line application that displays animated plasma effects in a window.
//!
//! This program creates a window that shows colorful, animated plasma patterns
//! using various geometric shapes and color palettes. The display can be
//! configured using command-line arguments to specify window dimensions,
//! pattern shapes, and color schemes.
//!
//! # Usage
//! ```bash
//! plasma [OPTIONS]
//!
//! Options:
//!   -w, --width <WIDTH>    Screen width in pixels [default: 1366]
//!   -h, --height <HEIGHT>  Screen height in pixels [default: 768]
//!   -s, --shape <SHAPE>    Plasma shape [default: ripple]
//!   -p, --palette <PALETTE>  Plasma color palette [default: rainbow]
//! ```
//!
//! The program will run until the Escape key is pressed or the window is closed.
use crate::plasma::Plasma;
use clap::Parser;
use minifb::{Window, WindowOptions};
use std::time::Instant;

mod plasma;

#[doc(hidden)]
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct PlasmaArgs {
    #[arg(short, long, default_value_t = 1366, help = "Screen width in pixels")]
    width: usize,

    #[arg(short, long, default_value_t = 768, help = "Screen height in pixels")]
    height: usize,

    #[arg(
        short,
        long,
        value_enum,
        default_value_t = plasma::Shape::Ripple,
        help = "Plasma shape"
    )]
    shape: plasma::Shape,

    #[arg(
        short,
        long,
        value_enum,
        default_value_t = plasma::Palette::Rainbow,
        help = "Plasma color palette"
    )]
    palette: plasma::Palette,
}

#[doc(hidden)]
fn run(plasma: Plasma, width: usize, height: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("Demo Effect", width, height, WindowOptions::default())?;

    let start_time = Instant::now();
    let mut buffer = vec![0; width * height];

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let time = start_time.elapsed().as_secs_f32();
        plasma.draw(&mut buffer, time);
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
    Ok(())
}

#[doc(hidden)]
fn main() {
    let args = PlasmaArgs::parse();
    let plasma = plasma::Plasma::new(args.width, args.height, args.shape, args.palette);

    if let Err(e) = run(plasma, args.width, args.height) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
