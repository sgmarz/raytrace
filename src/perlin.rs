// perlin.rs
// Perlin noise class
// Stephen Marz
// 15 Dec 2020

use crate::random::{random_double, random_int};
use crate::vector::Vec3;
use std::vec::Vec;

const POINT_COUNT: usize = 256;

pub struct Perlin {
	ranfloat: Vec<f64>,
	perm: [Vec<usize>; 3],
}

impl Default for Perlin {
	fn default() -> Self {
		Self::new()
	}
}

impl Perlin {
	pub fn new() -> Self {
		let mut ranfloat = Vec::with_capacity(POINT_COUNT);
		for _ in 0..POINT_COUNT {
			ranfloat.push(random_double(0.0, 1.0));
		}
		let perm_x = Self::perlin_generate_perm();
		let perm_y = Self::perlin_generate_perm();
		let perm_z = Self::perlin_generate_perm();
		Self {
			ranfloat,
			perm: [perm_x, perm_y, perm_z],
		}
	}

	pub fn noise(&self, p: &Vec3) -> f64 {
		let u = p.x() - p.x().floor();
		let v = p.y() - p.y().floor();
		let w = p.z() - p.z().floor();

		let u = u * u * (3.0 - 2.0 * u);
		let v = v * v * (3.0 - 2.0 * v);
		let w = w * w * (3.0 - 2.0 * w);

		let i = p.x().floor() as usize;
		let j = p.y().floor() as usize;
		let k = p.z().floor() as usize;
		let mut c = [[[0.0; 2]; 2]; 2];

		for di in 0..2 {
			for dj in 0..2 {
				for dk in 0..2 {
					c[di][dj][dk] = self.ranfloat[self.perm[0][(i + di) & 255] ^ self.perm[1][(j + dj) & 255] ^ self.perm[2][(k + dk) & 255]];
				}
			}
		}

		Self::trilinear_interpolation(c, u, v, w)
	}

	fn perlin_generate_perm() -> Vec<usize> {
		let mut p = Vec::with_capacity(POINT_COUNT);

		for i in 0..POINT_COUNT {
			p.push(i)
		}

		Self::permute(&mut p, POINT_COUNT);
		p
	}

	fn permute(p: &mut [usize], n: usize) {
		for i in 1..n {
			let target = random_int(0, i as i32) as usize;
			p.swap(i, target);
		}
	}

	fn trilinear_interpolation(perm: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
		let mut accum = 0.0;
		for i in 0usize..2 {
			for j in 0usize..2 {
				for k in 0usize..2 {
					let a = i as f64;
					let b = j as f64;
					let c = k as f64;
					accum += (a * u + (1.0 - a) * (1.0 - u)) * (b * v + (1.0 - b) * (1.0 - v)) * (c * w + (1.0 - c) * (1.0 - w)) * perm[i][j][k];
				}
			}
		}
		accum
	}
}
