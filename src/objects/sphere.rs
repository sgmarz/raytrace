
use crate::hitable::{Hitable, HitRecord};
use crate::vector::Vec3;
use crate::ray::Ray;
use std::ops::{Add, Sub};
use crate::material::Material;
use crate::bounding_box::AxisAlignedBoundingBox;


pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Material::new_lambertian(Vec3::new(1.0, 1.0, 1.0)))
    }
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material
        }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    fn get_uv(&self, point: &Vec3) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let theta = -&point.y().acos();
        let phi = -&point.z().atan2(point.x()) + pi;

        let u = phi / (2.0 * pi);
        let v = theta / pi;
        (u, v)
    }

}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin().sub(self.center());
        let a = ray.direction().len2();
        let half_b = oc.dot(ray.direction());
        let c = oc.len2() - self.radius() * self.radius();
    
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let t= root;
        let point = ray.at(t);
        let outward_normal = (point - self.center()) / self.radius;
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        let (u, v) = self.get_uv(&outward_normal);
        Some(HitRecord::new(point, normal, t, front_face, self.material.clone(), u, v))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        let output_box = AxisAlignedBoundingBox::new(
        self.center().sub(&Vec3::new(self.radius, self.radius, self.radius)),
        self.center().add(&Vec3::new(self.radius, self.radius, self.radius)));
        Some(output_box)
    }

}
