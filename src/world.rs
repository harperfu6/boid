use rand::Rng;

#[derive(Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn get_x(self) -> f32 {
        self.x
    }

    pub fn get_y(self) -> f32 {
        self.y
    }

    fn move_forward(&mut self, x: f32, y: f32) -> () {
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

#[derive(Clone, Copy)]
struct Vector {
    dx: f32,
    dy: f32,
}

impl Vector {}

#[derive(Clone, Copy)]
pub struct Boid {
    id: u32,
    point: Point,
    vector: Vector,
}

impl Boid {
    fn new(point: Point, vector: Vector, id: u32) -> Boid {
        Boid { id, point, vector }
    }

    pub fn get_point(self) -> Point {
        self.point.clone()
    }

    fn step_forward(&mut self, percent: f32) -> () {
        let x = self.vector.dx * percent;
        let y = self.vector.dy * percent;
        self.point.move_forward(x, y)
    }

    fn step(&mut self, seconds: f32) -> () {
        self.step_forward(seconds)
    }
}

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
            boid.step(seconds);
            self.boids[i] = boid;
        }
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }
}