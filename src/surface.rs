
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Default)]
pub struct Surface {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64
}

impl Surface {
    pub const fn new(point: Vec3, normal: Vec3, t: f64) -> Self {
        Self {
            point,
            normal,
            t
        }
    }
}

pub trait HitSurface {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Surface>;
}
