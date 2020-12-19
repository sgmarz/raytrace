// texture.rs
// Texturing functions
// Stephen Marz
// 15 Dec 2020

use crate::perlin::Perlin;
use crate::vector::{Color, Vec3};
use std::fs::File;
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

pub struct ImageTexture {
	data: Vec<(f64, f64, f64)>,
	width: usize,
	height: usize,
	bytes_per_scanline: usize,
}

impl ImageTexture {
	pub fn new(data: Vec<(f64, f64, f64)>, width: usize, height: usize, bytes_per_scanline: usize) -> Self {
		Self {
			data,
			width,
			height,
			bytes_per_scanline,
		}
	}

	pub fn from_file(fname: &str) -> Self {
		// The decoder is a build for reader and can be used to set various decoding options
		// via `Transformations`. The default output transformation is `Transformations::EXPAND
		// | Transformations::STRIP_ALPHA`.
		let decoder = png::Decoder::new(File::open(fname).unwrap());
		let (info, mut reader) = decoder.read_info().unwrap();
		let width = info.width as usize;
		let height = info.height as usize;
		// println!("Width {}, Height {}, total = {}", width, height, width * height);
		let bytes_per_scanline = width;
		// Allocate the output buffer.
		let mut buf = vec![0; info.buffer_size()];
		let mut data = Vec::<(f64, f64, f64)>::with_capacity(info.buffer_size() / 3);
		
		// Read the next frame. An APNG might contain multiple frames.
		while let Ok(_) = reader.next_frame(&mut buf) {
		// Inspect more details of the last read frame.
		// let in_animation = reader.info().frame_control.is_some();
			for i in (0..buf.len()).step_by(4) {
				let rgb = (buf[i] as f64 / 255.0, buf[i + 1] as f64 / 255.0, buf[i + 2] as f64 / 255.0);
				data.push(rgb);
			}
		}

		Self {
			data,
			width,
			height,
			bytes_per_scanline,
		}
	}
}

impl Texture for ImageTexture {
	fn value(&self, u: f64, v: f64, _point: &Vec3) -> Color {
		if self.data.len() == 0 {
			return Color::new(0.0, 1.0, 1.0);
		}

		// Clamp input texture coordinates to [0,1] x [1,0]
		let u = clamp(u, 0.0, 1.0);
		let v = 1.0 - clamp(v, 0.0, 1.0); // Flip V to image coordinates

		let mut i = (u * self.width as f64) as usize;
		let mut j = (v * self.height as f64) as usize;

		// Clamp integer mapping, since actual coordinates should be less than 1.0
		if i >= self.width {
			i = self.width - 1;
		}
		if j >= self.height {
			j = self.height - 1;
		}

		// println!("i = {}, j = {}", i, j);
		let pixel = self.data[j * self.width + i];

		Color::new(pixel.0, pixel.1, pixel.2)
	}
}

fn clamp(val: f64, min: f64, max: f64) -> f64 {
	if val < min {
		min
	} else if val > max {
		max
	} else {
		val
	}
}
