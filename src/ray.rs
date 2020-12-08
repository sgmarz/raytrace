
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
    pub fn get_origin(&self) -> Vec3 {
        Vec3::new(self.origin.get_x(), self.origin.get_y(), self.origin.get_z())
    }
    pub fn get_origin_mut(&mut self) -> &mut Vec3 {
        &mut self.origin
    }
    pub fn get_origin_ref(&self) -> &Vec3 {
        &self.origin
    }
    pub fn get_direction(&self) -> Vec3 {
        Vec3::new(self.direction.get_x(), self.direction.get_y(), self.direction.get_z())
    }
    pub fn get_direction_mut(&mut self) -> &mut Vec3 {
        &mut self.direction
    }
    pub fn get_direction_ref(&self) -> &Vec3 {
        &self.direction
    }
    pub fn at(&self, coeff: f64) -> Vec3 {
        let o = self.get_origin_ref();
        let d = self.get_direction_ref();
        Vec3::new(o.get_x() + coeff * d.get_x(),
            o.get_y() + coeff * d.get_y(),
            o.get_z() + coeff * d.get_z())
    }
    pub fn color(&self, clr: &Color) -> Color {
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (1.0 + unit_direction.get_y());
        clr.lerp(t)
    }
}