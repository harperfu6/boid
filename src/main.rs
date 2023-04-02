mod world;

use world::World;

use piston::WindowSettings;
use piston_window::{clear, polygon, PistonWindow, Transformed};

const NUM_BOIDS: u32 = 250;
const SIZE: u32 = 600;

const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]];

fn main() {
    let mut the_flock = World::new(NUM_BOIDS, SIZE as f32);

    let mut window: PistonWindow = WindowSettings::new("flock-of-boids", [SIZE, SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let i = 1.1;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, gfx, _| {
            clear([1.0, 1.0, 1.0, 1.0], gfx);

            the_flock.step(i);
            let boids = the_flock.get_boids();
            for i in 0..boids.len() {
                let boid = boids[i];
                let point = boid.get_point();

                let transform = context
                    .transform
                    .trans(point.get_x() as f64, point.get_y() as f64);
                polygon([1.0, 0.0, 1.0, 1.0], BOID_BOD, transform, gfx)
            }
        });
    }
}
