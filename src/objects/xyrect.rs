
use crate::vector::Vec3;
use crate::material::Material;
use crate::hitable::{Hitable, HitRecord};
use crate::bounding_box::AxisAlignedBoundingBox;
use crate::ray::Ray;

pub struct XyRect {
    material: Material,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k
        }
    }
}

impl Hitable for XyRect {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t*r.direction().x();
        let y = r.origin().y() + t*r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0)/(self.x1 - self.x0);
        let v = (y - self.y0)/(self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let front_face = r.direction().dot(&outward_normal) < 0.0;
		let normal = if front_face {
			outward_normal
		} else {
			-outward_normal
		};
        
        let p = r.at(t);
        Some(HitRecord::new(p, normal, t, front_face, self.material.clone(), u, v))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
		let output_box = AxisAlignedBoundingBox::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001)
		);
		Some(output_box)
	}

	fn translate(&mut self, x: f64, y: f64, z: f64) {
        self.x0 += x;
        self.x1 += x;
        self.y0 += y;
        self.y1 += y;
        self.k += z;
	}
}
