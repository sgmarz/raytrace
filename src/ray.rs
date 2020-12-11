
use crate::vector::{Color, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(o: &Vec3, d: &Vec3) -> Self {
        Self {
            origin: Vec3::new(o["x"], o["y"], o["z"]),
            direction: Vec3::new(d["x"], d["y"], d["z"])
        }
    }

    pub fn trace(&self) -> Color {
        let dir = self.direction.normalize();
        let t = 0.5 * (dir["y"] + 1.0);
        let c0 = Color::new(t, t, t);
        let c1 = Color::new(t * 0.5, t * 0.7, t * 1.0);
        c0 * c1
    }
}


