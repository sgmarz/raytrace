// moving_sphere.rs
// Time-biased sphere object
// Stephen Marz
// 15 Dec 2020

use crate::{bounding_box::AxisAlignedBoundingBox,
            hitable::{HitRecord, Hitable},
            material::Material,
            ray::Ray,
            vector::Vec3};
use std::ops::{Add, Sub};

pub struct MovingSphere {
	center0:  Vec3,
	center1:  Vec3,
	time0:    f64,
	time1:    f64,
	radius:   f64,
	material: Material
}

impl MovingSphere {
	pub fn new(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, material: Material) -> Self {
		Self { center0,
		       center1,
		       time0,
		       time1,
		       radius,
		       material }
	}

	pub fn center(&self, time: f64) -> Vec3 {
		self.center0 + &((self.center1 - &self.center0) * ((time - self.time0) / (self.time1 - self.time0)))
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

impl Hitable for MovingSphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = ray.origin().sub(&self.center(ray.time()));
		let a = ray.direction().len2();
		let half_b = oc.dot(ray.direction());
		let c = oc.len2() - self.radius * self.radius;

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

		let t = root;
		let point = ray.at(t);
		let outward_normal = (point - &self.center(ray.time())) / self.radius;
		let front_face = ray.direction().dot(&outward_normal) < 0.0;
		let material = self.material.clone();
		let (u, v) = self.get_uv(&outward_normal);
		let rec = HitRecord::new(point, outward_normal, t, front_face, material, u, v);

		Some(rec)
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
		let box0 = AxisAlignedBoundingBox::new(
		                                       self.center(time0).sub(&Vec3::new(self.radius, self.radius, self.radius)),
		                                       self.center(time0).add(&Vec3::new(self.radius, self.radius, self.radius))
		);
		let box1 = AxisAlignedBoundingBox::new(
		                                       self.center(time1).sub(&Vec3::new(self.radius, self.radius, self.radius)),
		                                       self.center(time1).add(&Vec3::new(self.radius, self.radius, self.radius))
		);
		Some(box0.surrounding_box(&box1))
	}
}
