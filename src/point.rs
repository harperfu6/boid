use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::vector::Vector;

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn mean(points: Vec<Point>) -> Point {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for i in 0..points.len() {
            sum_x += points[i].x;
            sum_y += points[i].y;
        }
        let len: f32 = points.len() as f32;
        Point {
            x: sum_x / len,
            y: sum_y / len,
        }
    }

    pub fn get_x(self) -> f32 {
        self.x
    }

    pub fn get_y(self) -> f32 {
        self.y
    }

    pub fn bound(&mut self, width: f32, height: f32) -> () {
        let mut x = self.x % width;
        let mut y = self.y % height;

        if x < 0.0 {
            x += width;
        }

        if y < 0.0 {
            y += height;
        }

        self.x = x;
        self.y = y;
    }

    pub fn vector_to(&self, other: &Point) -> Vector {
        Vector {
            dx: other.x - self.x,
            dy: other.y - self.y,
        }
    }

    pub fn move_forward(&mut self, x: f32, y: f32) -> () {
        self.x += x;
        self.y += y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn means() {}
}
