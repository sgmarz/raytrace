// png.rs
// PNG scene writer
// Stephen Marz
// 9 Dec 2020

use crate::vector::Color;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::vec::Vec;

pub struct Row {
	cols: Vec<Color>,
}

impl Default for Row {
	fn default() -> Self {
		Self {
			cols: Vec::new(),
		}
	}
}

impl Row {
	pub fn resize_width(&mut self, new_width: usize) {
		self.cols.resize_with(new_width as usize, Default::default);
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

pub struct PngPicture {
	samples: u32,
	width: u32,
	height: u32,
	rows: Vec<Row>,
}

impl PngPicture {
	pub fn new(width: u32, height: u32, samples: u32) -> Self {
		let mut r = Self {
			samples,
			width,
			height,
			rows: Vec::new(),
		};
		r.resize(width, height);
		r
	}

	pub fn write_file(&self, fname: &str) -> Result<usize, Error> {
		let wd = BufWriter::new(File::create(fname)?);
		let mut bytes_written = 0;

		let scale = 1.0 / self.samples as f64;

		let mut encoder = png::Encoder::new(wd, self.width, self.height);
		encoder.set_color(png::ColorType::RGB);
		encoder.set_depth(png::BitDepth::Eight);
		let writer = encoder.write_header().unwrap();
		let mut swriter = writer.into_stream_writer();

		for row in (0..self.height).rev() {
			for col in 0..self.width {
				let px = self.get_pixel(col, row);
				let pxr = scale * px.r();
				let pxg = scale * px.g();
				let pxb = scale * px.b();
				let r = (255.0 * clamp(pxr, 0.0, 1.0)) as u8;
				let g = (255.0 * clamp(pxg, 0.0, 1.0)) as u8;
				let b = (255.0 * clamp(pxb, 0.0, 1.0)) as u8;
				let data = [r, g, b];
				bytes_written += swriter.write(&data).unwrap();
			}
		}
		Ok(bytes_written)
	}

	pub fn get_pixel(&self, x: u32, y: u32) -> Color {
		let p = &self.rows[y as usize].cols[x as usize];
		Color::new(p[0], p[1], p[2])
	}
	pub fn set_pixel(&mut self, x: u32, y: u32, pixel: &Color) {
		let p = &mut self.rows[y as usize].cols[x as usize];
		p[0] = pixel[0];
		p[1] = pixel[1];
		p[2] = pixel[2];
	}
	pub fn get_width(&self) -> u32 {
		self.width
	}
	pub fn get_height(&self) -> u32 {
		self.height
	}
	pub fn resize(&mut self, new_width: u32, new_height: u32) {
		self.width = new_width;
		self.height = new_height;
		let resize_width = || {
			let mut r = Row::default();
			r.resize_width(new_width as usize);
			r
		};
		self.rows.resize_with(new_height as usize, resize_width);
	}
}
