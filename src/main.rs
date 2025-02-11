//! A graphical plasma effect visualizer with interactive controls.
//!
//! This program generates animated plasma patterns in a window with real-time controls
//! for adjusting the visualization parameters.
//!
//! # Controls
//! - `Space`: Cycle through color palettes
//! - `Left/Right`: Change pattern shape
//! - `Up/Down`: Adjust pattern scale
//! - `Escape/Q`: Exit program
//!
//! # Command Line Arguments
//! ```text
//! Options:
//!   -w, --width <WIDTH>      Screen width in pixels [default: 1366]
//!   -h, --height <HEIGHT>    Screen height in pixels [default: 768]
//!   -s, --shape <SHAPE>      Initial plasma shape [default: ripple]
//!   -p, --palette <PALETTE>  Initial color palette [default: rainbow]
//!   -x, --scale <SCALE>      Pattern scale factor [default: 10.0]
//! ```
use crate::plasma::Plasma;
use clap::Parser;
use minifb::{Key, Window, WindowOptions};
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

    #[arg(
        short = 'x',
        long,
        default_value_t = 10.0,
        help = "Scale factor that controls the density/size of the plasma patterns"
    )]
    scale: f32,
}

#[doc(hidden)]
fn run(mut plasma: Plasma, width: usize, height: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("Plasma", width, height, WindowOptions::default())?;

    let start_time = Instant::now();
    let mut last_key_time = Instant::now();
    let mut buffer = vec![0; width * height];

    // Minimum time (in seconds) between key presses
    // Oddly, the minifb functions set_key_repeat() and set_key_delay() don't work as expected so
    // we resorted to manual key delay handling.
    const KEY_DELAY: f32 = 0.15;

    while window.is_open() {
        let current_time = Instant::now();
        let key_elapsed = current_time.duration_since(last_key_time).as_secs_f32();

        if key_elapsed >= KEY_DELAY {
            if let Some(key) = window.get_keys().first() {
                match key {
                    Key::Escape => std::process::exit(0),
                    Key::Q => std::process::exit(0),
                    Key::Space => plasma.next_palette(),
                    Key::Up => plasma.decrease_scale(),
                    Key::Down => plasma.increase_scale(),
                    Key::Left => plasma.prev_shape(),
                    Key::Right => plasma.next_shape(),
                    _ => {}
                }
                last_key_time = current_time;
            }
        }

        let time = start_time.elapsed().as_secs_f32();
        plasma.draw(&mut buffer, time);
        window.update_with_buffer(&buffer, width, height)?;
    }
    Ok(())
}

#[doc(hidden)]
fn main() {
    let args = PlasmaArgs::parse();
    let plasma = plasma::Plasma::new(
        args.width,
        args.height,
        args.shape,
        args.palette,
        args.scale,
    );

    if let Err(e) = run(plasma, args.width, args.height) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
