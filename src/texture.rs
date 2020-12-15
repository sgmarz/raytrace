// texture.rs
// Texturing functions
// Stephen Marz
// 15 Dec 2020

use crate::perlin::Perlin;
use crate::vector::{Color, Vec3};
use std::sync::Arc;

pub trait Texture {
	fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

// TEXTURES

// Solid color
#[derive(Default)]
pub struct SolidColor {
	color_value: Color,
}
unsafe impl Send for SolidColor {}
unsafe impl Sync for SolidColor {}

impl SolidColor {
	pub fn new(color_value: Color) -> Self {
		Self {
			color_value,
		}
	}

	pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
		Self {
			color_value: Color::new(r, g, b),
		}
	}
}

impl Texture for SolidColor {
	fn value(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
		// A solid color doesn't care about where the ray is being shot
		self.color_value
	}
}

// Checkered texture

pub struct CheckeredTexture {
	odd: Arc<dyn Texture + Send + Sync>,
	even: Arc<dyn Texture + Send + Sync>,
}

unsafe impl Send for CheckeredTexture {}
unsafe impl Sync for CheckeredTexture {}

impl CheckeredTexture {
	pub fn new(odd: Arc<dyn Texture + Send + Sync>, even: Arc<dyn Texture + Send + Sync>) -> Self {
		Self {
			odd,
			even,
		}
	}

	pub fn new_color(odd: Vec3, even: Vec3) -> Self {
		let odd = Arc::new(SolidColor::new(odd));
		let even = Arc::new(SolidColor::new(even));
		Self::new(odd, even)
	}
}

impl Texture for CheckeredTexture {
	fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
		let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
		if sines < 0.0 {
			self.odd.value(u, v, p)
		} else {
			self.even.value(u, v, p)
		}
	}
}

// Perlin noise texture

pub struct NoiseTexture {
	perlin: Perlin,
}

unsafe impl Send for NoiseTexture {}
unsafe impl Sync for NoiseTexture {}

impl NoiseTexture {
	pub fn new() -> Self {
		Self {
			perlin: Perlin::new(),
		}
	}
}

impl Texture for NoiseTexture {
	fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
		Vec3::new(1.0, 1.0, 1.0) * self.perlin.noise(p)
	}
}
