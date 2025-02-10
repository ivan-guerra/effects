//! Command-line demo effects renderer with configurable display options
//!
//! This module provides the main entry point and CLI interface for running
//! various demo scene effects.
//!
//! # Usage
//!
//! ```bash
//! effects [OPTIONS] <COMMAND>
//!
//! Commands:
//!   plasma    Run a plasma effect with configurable shape and palette
//!   help      Print help information
//!
//! Options:
//!   -w, --width <WIDTH>    Screen width in pixels [default: 1366]
//!   -h, --height <HEIGHT>  Screen height in pixels [default: 768]
//!   -v, --version          Print version information
//! ```
//!
//! The application runs in a window and can be closed by pressing ESC.
//! Each effect has its own set of configuration options accessible via
//! subcommands.
use crate::common::DemoEffect;
use crate::effects::plasma;
use clap::{Args, Parser, Subcommand};
use minifb::{Window, WindowOptions};
use std::time::Instant;

mod common;
mod effects;

#[doc(hidden)]
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = 1366, help = "Screen width in pixels")]
    width: usize,

    #[arg(short, long, default_value_t = 768, help = "Screen height in pixels")]
    height: usize,
}

#[doc(hidden)]
#[derive(Subcommand)]
enum Commands {
    Plasma(PlasmaArgs),
}

#[doc(hidden)]
#[derive(Args)]
#[command(about = "Plasma effect configuration")]
struct PlasmaArgs {
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
fn run(
    demo_effect: Box<dyn DemoEffect>,
    width: usize,
    height: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("Demo Effect", width, height, WindowOptions::default())?;

    let start_time = Instant::now();
    let mut buffer = vec![0; width * height];

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let time = start_time.elapsed().as_secs_f32();
        demo_effect.draw(&mut buffer, time);
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
    Ok(())
}

#[doc(hidden)]
fn create_effect(args: &Cli) -> Box<dyn DemoEffect> {
    match &args.command {
        Commands::Plasma(plasma_args) => Box::new(plasma::Config::new(
            args.width,
            args.height,
            plasma_args.shape.clone(),
            plasma_args.palette.clone(),
        )),
    }
}

#[doc(hidden)]
fn main() {
    let args = Cli::parse();
    let demo_effect = create_effect(&args);

    if let Err(e) = run(demo_effect, args.width, args.height) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
