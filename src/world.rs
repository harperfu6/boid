use crate::boid::Boid;
use crate::point::Point;
use crate::vector::Vector;
use rand::Rng;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
}

#[cfg(not(target_arch = "wasm32"))]
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

const SIGHT: f32 = 25.0;
const GRID_GAP: f32 = 8.0;
const FIELD_OF_VIEW: f32 = std::f32::consts::PI * 3.0 / 4.0;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
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
            let neighbors = self.get_visible_neighbors(&boid);
            boid.step(seconds, &neighbors);
            // boid.step(seconds, &self.boids);
            boid.bound(self.width, self.height);
            self.boids[i] = boid;
        }
    }

    pub fn get_boids(&self) -> Vec<JsValue> {
        self.boids
            .clone()
            .iter()
            .map(|b| serde_wasm_bindgen::to_value(b).unwrap())
            .collect()
    }

    pub fn get_visible_neighbors(&self, boid: &Boid) -> Vec<JsValue> {
        let grid = Grid {
            x: (boid.point.get_x() / SIGHT).floor(),
            y: (boid.point.get_y() / SIGHT).floor(),
        };
        self.boids
            .iter()
            .cloned()
            .filter(|b| {
                if b.id == boid.id {
                    return false;
                }

                let other_grid = Grid {
                    x: (b.point.get_x() / SIGHT).floor(),
                    y: (b.point.get_y() / SIGHT).floor(),
                };

                if (grid.x - other_grid.x).abs() + (grid.y - other_grid.y).abs() > GRID_GAP {
                    return false;
                }

                let vector: Vector = boid.point.vector_to(&b.point);
                if vector.get_length() > SIGHT {
                    return false;
                }

                if vector.radial_distance(boid.vector) > FIELD_OF_VIEW {
                    return false;
                }

                true
            })
            .collect::<Vec<Boid>>()
            .iter()
            .map(|b| serde_wasm_bindgen::to_value(b).unwrap())
            .collect()
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
            let neighbors = self.get_visible_neighbors(&boid);
            boid.step(seconds, &neighbors);
            // boid.step(seconds, &self.boids);
            boid.bound(self.width, self.height);
            self.boids[i] = boid;
        }
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn get_visible_neighbors(&self, boid: &Boid) -> Vec<Boid> {
        let grid = Grid {
            x: (boid.point.get_x() / SIGHT).floor(),
            y: (boid.point.get_y() / SIGHT).floor(),
        };
        self.boids
            .iter()
            .cloned()
            .filter(|b| {
                if b.id == boid.id {
                    return false;
                }

                let other_grid = Grid {
                    x: (b.point.get_x() / SIGHT).floor(),
                    y: (b.point.get_y() / SIGHT).floor(),
                };

                if (grid.x - other_grid.x).abs() + (grid.y - other_grid.y).abs() > GRID_GAP {
                    return false;
                }

                let vector: Vector = boid.point.vector_to(&b.point);
                if vector.get_length() > SIGHT {
                    return false;
                }

                if vector.radial_distance(boid.vector) > FIELD_OF_VIEW {
                    return false;
                }

                true
            })
            .collect::<Vec<Boid>>()
    }
}
