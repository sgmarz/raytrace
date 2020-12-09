
use crate::vector::Vec3;
use crate::color::Color;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn at(&self, coeff: f64) -> Vec3 {
        Vec3::new(self.origin[0] + coeff * self.origin[0],
            self.origin[1] + coeff * self.origin[1],
            self.origin[2] + coeff * self.origin[2])
    }
    
    pub fn color(&self, clr: &Color) -> Color {
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (1.0 + unit_direction[1]);
        clr.lerp(t)
    }
}


