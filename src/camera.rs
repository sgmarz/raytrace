
use crate::vector::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        
        let w = (look_from - &look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - &(horizontal / 2.0) - &(vertical / 2.0) - &w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin().clone(), self.lower_left_corner + &(self.horizontal * s) + &(self.vertical * t) - self.origin())
    }
}

