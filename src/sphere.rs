
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
        let dlen = ray.direction.len();
        let oclen = oc.len();
        let a = dlen * dlen;
        let half_b = oc.dot(&ray.direction);
        let c = oclen * oclen - self.radius * self.radius;
    
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
        let point = ray.at(root);
        let normal = (point - self.center) / Vec3::new_scalar(self.radius);
        let rec = Surface::new(point, normal, root);
        Some(rec)
    }
}
