
use crate::vector::Vec3;
use crate::color::Color;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn at(&self, coeff: f64) -> Vec3 {
        Vec3::new(self.origin["x"] + coeff * self.origin["x"],
            self.origin["y"] + coeff * self.origin["y"],
            self.origin["z"] + coeff * self.origin["z"])
    }
    
    pub fn color(&self, clr: &Color) -> Color {
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (1.0 + unit_direction["y"]);
        clr.lerp(t)
    }
}


