use crate::texture::SolidColor;
use crate::{bounding_box::AxisAlignedBoundingBox, material::Material, ray::Ray, vector::Vec3};
use std::{sync::Arc, vec::Vec};

pub struct HitRecord {
	point: Vec3,
	normal: Vec3,
	t: f64,
	material: Material,
	front_face: bool,
	u: f64,
	v: f64,
}

impl Default for HitRecord {
	fn default() -> Self {
		Self::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false, Material::new_lambertian(Arc::new(SolidColor::from_rgb(0.5, 0.25, 0.75))), 0.0, 0.0)
	}
}

impl HitRecord {
	pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool, material: Material, u: f64, v: f64) -> Self {
		Self {
			point,
			normal,
			t,
			material,
			front_face,
			u,
			v,
		}
	}

	pub fn material(&self) -> &Material {
		&self.material
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

	pub fn uv(&self) -> (f64, f64) {
		(self.u, self.v)
	}

	pub fn u(&self) -> f64 {
		self.u
	}

	pub fn v(&self) -> f64 {
		self.v
	}

	pub fn front_face(&self) -> bool {
		self.front_face
	}
}

pub trait Hitable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
	fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox>;
	fn translate(&mut self, x: f64, y: f64, z: f64);
}

#[derive(Default, Clone)]
pub struct HitList {
	objects: Vec<Arc<dyn Hitable + Send + Sync>>,
}

impl HitList {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
		}
	}

	pub fn add(&mut self, obj: Arc<dyn Hitable + Send + Sync>) {
		self.objects.push(obj);
	}

	pub fn objects(&self) -> &Vec<Arc<dyn Hitable + Send + Sync>> {
		&self.objects
	}

	pub fn objects_mut(&mut self) -> &mut Vec<Arc<dyn Hitable + Send + Sync>> {
		&mut self.objects
	}

	pub fn len(&self) -> usize {
		self.objects.len()
	}

	pub fn is_empty(&self) -> bool {
		self.objects.is_empty()
	}

	pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut closest_rec = HitRecord::default();
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for obj in self.objects.iter() {
			if let Some(rec) = obj.hit(ray, t_min, closest_so_far) {
				hit_anything = true;
				closest_so_far = rec.t();
				closest_rec = rec;
			}
		}

		if hit_anything {
			Some(closest_rec)
		} else {
			None
		}
	}

	pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
		if self.is_empty() {
			return None;
		}

		let mut first_box = true;
		let mut output_box = AxisAlignedBoundingBox::default();
		for object in self.objects.iter() {
			if let Some(aabb) = object.bounding_box(time0, time1) {
				output_box = if first_box {
					aabb
				} else {
					output_box.surrounding_box(&aabb)
				};
				first_box = false;
			} else {
				return None;
			}
		}
		Some(output_box)
	}
}
