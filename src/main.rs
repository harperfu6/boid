use piston::WindowSettings;
use piston_window::{clear, PistonWindow};

const NUM_BOIDS: u32 = 250;
const SIZE: u32 = 600;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("flock-of-boids", [SIZE, SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| clear([0.5, 1.0, 0.5, 1.0], g));
    }
}
