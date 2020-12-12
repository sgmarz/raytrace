
use crate::surface::{Surface, HitSurface};
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(ctr: &Vec3, radius: f64) -> Self {
        let center = Vec3::new(ctr[0], ctr[1], ctr[2]);
        Self {
            center,
            radius
        }
    }
}

impl HitSurface for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Surface> {
        let oc = ray.origin.rsub(&self.center);
        let a = ray.direction.len() * ray.direction.len();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len() * oc.len() - self.radius * self.radius;
    
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = Surface::default();
        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.normal = (rec.point - self.center) / Vec3::new_scalar(self.radius);
        Some(rec)
    }
}
