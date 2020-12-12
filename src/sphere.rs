
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::material::Material;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: dyn Material
}

impl Sphere {
    pub fn new(ctr: &Vec3, radius: f64, material: dyn Material) -> Self {
        let center = Vec3::new(ctr[0], ctr[1], ctr[2]);
        Self {
            center,
            radius,
            material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        if root < t_min || root > t_max {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let sradius = Vec3::new_scalar(self.radius);
        let point = ray.at(root);
        let normal = point.rsub(&self.center).rdiv(&sradius);
        let mut sfc = HitRecord::new(point, normal, &self.material, root);
        sfc.set_face_normal(&ray, &normal);
        Some(sfc)
    }
}
