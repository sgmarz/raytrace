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

	pub fn noise(&self, point: &Vec3) -> f64 {
		let i = (4.0 * point.x()) as usize & (POINT_COUNT - 1);
		let j = (4.0 * point.y()) as usize & (POINT_COUNT - 1);
		let k = (4.0 * point.z()) as usize & (POINT_COUNT - 1);

		return self.ranfloat[self.perm[0][i] ^ self.perm[1][j] ^ self.perm[2][k]];
	}

	fn perlin_generate_perm() -> Vec<usize> {
		let mut p = Vec::with_capacity(POINT_COUNT);

		for i in 0..POINT_COUNT {
			p.push(i)
		}

		Self::permute(&mut p, POINT_COUNT);
		p
	}

	fn permute(p: &mut Vec<usize>, n: usize) {
		for i in (1..n).rev() {
			let target = random_int(0, i as i32) as usize;
			let tmp = p[i];
			p[i] = p[target];
			p[target] = tmp;
		}
	}
}
