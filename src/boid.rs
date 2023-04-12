use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::constants::PI_X_2;
use crate::point::Point;
use crate::vector::Vector;

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Boid {
    pub id: u32,
    pub point: Point,
    pub vector: Vector,
}

impl Boid {
    pub fn new(point: Point, vector: Vector, id: u32) -> Boid {
        Boid { id, point, vector }
    }

    pub fn get_point(self) -> Point {
        self.point.clone()
    }

    pub fn get_angle(&self) -> f32 {
        self.vector.get_angle()
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.vector.set_angle(angle);
    }

    pub fn bound(&mut self, width: f32, height: f32) -> () {
        self.point.bound(width, height);
    }

    fn step_forward(&mut self, percent: f32) -> () {
        let x = self.vector.dx * percent;
        let y = self.vector.dy * percent;
        self.point.move_forward(x, y)
    }

    // pub fn step(&mut self, seconds: f32, neighbors: &Vec<Boid>) -> () {
    pub fn step(&mut self, seconds: f32, _neighbors: &Vec<JsValue>) -> () {
        let neighbors: Vec<Boid> = _neighbors
            .iter()
            .map(|jv| serde_wasm_bindgen::from_value::<Boid>(jv.clone()).unwrap())
            .collect();

        if neighbors.len() > 0 {
            let mut vectors: Vec<Vector> = Vec::new();

            // separation
            // Method checks for nearby boids and steers away
            let mut separation = Vector::mean(
                neighbors
                    .iter()
                    .map(|b| self.point.vector_to(&b.point))
                    .collect::<Vec<Vector>>(),
            );
            separation.set_length(separation.get_length() + 15f32);
            vectors.push(separation);

            // cohesion
            // For the average position (i.e. center) of all nearby boids, calculate steering vector towards that position
            let average_location =
                Point::mean(neighbors.iter().map(|b| b.point).collect::<Vec<Point>>());
            vectors.push(self.point.vector_to(&average_location));

            // alignment
            // For every nearby boid in the system, calculate the average velocity
            let average_heading = Vector::mean(
                neighbors
                    .iter()
                    .map(|b| {
                        let mut v = Vector { dx: 1.0, dy: 0.0 };
                        v.set_angle(b.vector.get_angle());
                        v.set_length(25f32);
                        v
                    })
                    .collect::<Vec<Vector>>(),
            );
            vectors.push(average_heading);

            let final_vector = Vector::mean(vectors);
            self.turn_to(final_vector.get_angle(), 0.02);
        }

        self.step_forward(seconds)
    }

    fn turn_to(&mut self, mut heading: f32, percent: f32) {
        let angle = self.get_angle();
        if heading < angle {
            heading += PI_X_2;
        }

        let mut diff = heading - angle;

        if diff >= std::f32::consts::PI {
            diff = diff - PI_X_2;
        }

        self.set_angle(angle + diff * percent);
    }
}
