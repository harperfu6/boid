use crate::boid::Boid;
use crate::point::Point;
use crate::vector::Vector;
use rand::Rng;

pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
}

struct Grid {
    x: f32,
    y: f32,
}

const MAX_VELOCITY: f32 = 2.0;
const MIN_VELOCITY: f32 = 0.5;

impl World {
    pub fn new(total_boids: u32, size: f32) -> World {
        let mut boids = Vec::new();
        let step = size / total_boids as f32;
        let mut rng = rand::thread_rng();
        for i in 0..total_boids {
            let v = i as f32 * step + 1f32;
            let point = Point::new(
                rng.gen_range(MIN_VELOCITY..v),
                rng.gen_range(MIN_VELOCITY..v),
            );
            let vector = Vector {
                dx: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
                dy: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
            };
            boids.push(Boid::new(point, vector, i));
        }

        World {
            width: size,
            height: size,
            boids,
        }
    }

    pub fn step(&mut self, seconds: f32) -> () {
        for i in 0..self.boids.len() {
            let mut boid = self.boids[i];
            boid.bound(self.width, self.height);
            boid.step(seconds, &self.boids);
            self.boids[i] = boid;
        }
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    // pub fn get_visible_neighbors(&self) -> Vec<Boid> {

    // }
}
