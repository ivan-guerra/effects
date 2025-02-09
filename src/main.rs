use crate::common::DemoEffect;
use crate::effects::plasma;
use minifb::{Window, WindowOptions};
use std::time::Instant;

mod common;
mod effects;

const WIDTH: usize = 1366;
const HEIGHT: usize = 768;

fn run(demo_effect: Box<dyn DemoEffect>) -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("Demo Effect", WIDTH, HEIGHT, WindowOptions::default())?;

    let start_time = Instant::now();
    let mut buffer = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let time = start_time.elapsed().as_secs_f32();
        demo_effect.draw(&mut buffer, time);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
    Ok(())
}

fn main() {
    let plasma_effect = Box::new(plasma::Config::new(
        WIDTH,
        HEIGHT,
        plasma::Shape::Ripple,
        plasma::Palette::Rainbow,
    ));

    if let Err(e) = run(plasma_effect) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
