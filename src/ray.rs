
use crate::vector::{Color, Vec3};
use crate::hitable::Hitable;

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

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::new(self.origin["x"] + t * self.direction["x"],
                  self.origin["y"] + t * self.direction["y"],
                  self.origin["z"] + t * self.direction["z"]
            )
    }

    pub fn color(&self, rec: &Surface) -> Color {
        let clr = Vec3::new_scalar(0.5).rmul(&rec.normal.radd(&Vec3::new(1.0, 1.0, 1.0)));
        clr
    }
}