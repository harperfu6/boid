use crate::constants::PI_X_2;

#[derive(Clone, Copy)]
pub struct Vector {
    pub dx: f32,
    pub dy: f32,
}

impl Vector {
    pub fn mean(vectors: Vec<Vector>) -> Vector {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for i in 0..vectors.len() {
            sum_x += vectors[i].dx;
            sum_y += vectors[i].dy;
        }
        let len: u32 = vectors.len().try_into().unwrap();
        Vector {
            dx: sum_x / len as f32,
            dy: sum_y / len as f32,
        }
    }

    pub fn get_length(&self) -> f32 {
        let dx = self.dx;
        let dy = self.dy;

        (dx * dx + dy * dy).sqrt()
    }

    pub fn set_length(&mut self, value: f32) {
        let stretch = value / self.get_length();

        self.dx *= stretch;
        self.dy *= stretch;
    }

    pub fn set_angle(&mut self, angle: f32) {
        let length = self.get_length();
        let rise = angle.sin() * length;
        let run = angle.cos() * length;

        self.dy = rise;
        self.dx = run;
    }

    pub fn get_angle(&self) -> f32 {
        let mut angle = self.dy.atan2(self.dx);

        if angle < 0.0 {
            angle += PI_X_2;
        }

        angle
    }

    pub fn radial_distance(&self, other: Vector) -> f32 {
        let diff = (other.get_angle() - self.get_angle()).abs();
        let diff2 = PI_X_2 - diff;

        return diff.min(diff2);
    }
}
