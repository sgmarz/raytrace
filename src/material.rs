// material.rs
// Material routines
// Stephen Marz
// 15 Dec 2020

use crate::hitable::HitRecord;
use crate::random::{random_in_unit_sphere, random_unit_vector};
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vector::{Color, Vec3};
use std::ops::{Add, Mul, Neg, Sub};
use std::sync::Arc;

#[derive(Copy, Clone)]
pub enum MaterialType {
	Lambertian,
	Metal,
	DiElectric,
}

#[derive(Clone)]
pub struct Material {
	material_type: MaterialType,
	albedo: Arc<dyn Texture + Send + Sync>,
	fuzz: f64,
	ir: f64,
}

impl Material {
	pub fn new_lambertian(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
		Self {
			material_type: MaterialType::Lambertian,
			albedo,
			fuzz: 0.0,
			ir: 0.0,
		}
	}

	pub fn new_metal(albedo: Arc<dyn Texture + Send + Sync>, fuzz: f64) -> Self {
		Self {
			material_type: MaterialType::Metal,
			albedo,
			fuzz: if fuzz < 1.0 {
				fuzz
			} else {
				1.0
			},
			ir: 0.0,
		}
	}

	pub fn new_dielectric(ir: f64) -> Self {
		Self {
			material_type: MaterialType::DiElectric,
			albedo: Arc::new(SolidColor::default()),
			fuzz: 0.0,
			ir,
		}
	}

	pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
		match self.material_type {
			MaterialType::Lambertian => self.scatter_lambertian(ray, rec),
			MaterialType::Metal => self.scatter_metal(ray, rec),
			MaterialType::DiElectric => self.scatter_dielectric(ray, rec),
		}
	}

	fn scatter_lambertian(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
		let scatter_direction = rec.normal().add(&random_unit_vector());
		let scatter_direction = if scatter_direction.near_zero() {
			rec.normal()
		} else {
			&scatter_direction
		};
		let scattered = Ray::new(rec.point().clone(), scatter_direction.clone(), ray.time());
		let attenuation = self.albedo.value(rec.u(), rec.v(), rec.point());
		Some((attenuation, scattered))
	}

	fn scatter_metal(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
		let reflected = reflect(&ray.direction().unit(), rec.normal());
		let scattered = Ray::new(rec.point().clone(), reflected + &(random_in_unit_sphere() * self.fuzz), ray.time());
		let attenuation = self.albedo.clone();
		if scattered.direction().dot(rec.normal()) > 0.0 {
			Some((attenuation.value(rec.u(), rec.v(), rec.point()), scattered))
		} else {
			None
		}
	}

	fn scatter_dielectric(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
		let attenuation = Color::new(1.0, 1.0, 1.0);
		let refraction_ratio = if rec.front_face() {
			1.0 / self.ir
		} else {
			self.ir
		};

		let unit_direction = ray.direction().unit();
		let refracted = refract(&unit_direction, rec.normal(), refraction_ratio);

		let scattered = Ray::new(rec.point().clone(), refracted, ray.time());
		Some((attenuation, scattered))
	}
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
	return v.sub(&(n.mul(v.dot(n) * 2.0)));
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = uv.neg().dot(n).min(1.0);
	let r_out_perp = uv.add(&n.mul(cos_theta)) * etai_over_etat;
	let perp = (1.0 - r_out_perp.len2()).abs().sqrt();
	let r_out_parallel = n.mul(perp);
	r_out_perp + &r_out_parallel
}
