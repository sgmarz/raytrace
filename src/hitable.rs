
use crate::vector::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f64
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64) -> Self {
        Self {
            point,
            normal,
            t
        }
    }

    pub fn point(&self) -> &Vec3 {
        &self.point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

