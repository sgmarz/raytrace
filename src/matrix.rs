
use std::ops::{Index, IndexMut, Mul, MulAssign};
use crate::vector::Vec3;

pub struct Mat3 {
    m: [f64; 9]
}

impl Mat3 {
    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        self[2] += x;
        self[5] += y;
        self[8] += z;
    }
    pub fn mul_vec(&self, vec: &Vec3) -> Vec3 {
        Vec3::new(self.m[0] * vec[0] + self.m[1] * vec[1] + self.m[2] * vec[2],
            self.m[3] * vec[0] + self.m[4] * vec[1] + self.m[5] * vec[2],
            self.m[6] * vec[0] + self.m[7] * vec[1] + self.m[8] * vec[2]
        )
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self {
            m: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        if self.m.len() > idx {
            &self.m[idx]
        }
        else {
            panic!("Matrix index out of bounds: {} / 9", idx);
        }
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if self.m.len() > idx {
            &mut self.m[idx]
        }
        else {
            panic!("Matrix index out of bounds: {} / 9", idx);
        }
    }
}


impl Mul for Mat3 {
    type Output = Mat3;
    fn mul(self, other: Mat3) -> Self::Output {
        Mat3 {
            m: [self.m[0] * other.m[0] + self.m[1] * self.m[3] + self.m[2] * self.m[6], 
                self.m[0] * other.m[1] + self.m[1] * self.m[4] + self.m[2] * self.m[7],
                self.m[0] * other.m[2] + self.m[1] * self.m[5] + self.m[2] * self.m[8],

                self.m[3] * other.m[0] + self.m[4] * self.m[3] + self.m[5] * self.m[6], 
                self.m[3] * other.m[1] + self.m[4] * self.m[4] + self.m[5] * self.m[7],
                self.m[3] * other.m[2] + self.m[4] * self.m[5] + self.m[5] * self.m[8],

                self.m[6] * other.m[0] + self.m[7] * self.m[3] + self.m[8] * self.m[6], 
                self.m[6] * other.m[1] + self.m[7] * self.m[4] + self.m[8] * self.m[7],
                self.m[6] * other.m[2] + self.m[7] * self.m[5] + self.m[8] * self.m[8],
                ]
        }
    }
}


impl MulAssign for Mat3 {
    fn mul_assign(&mut self, other: Mat3) {
        self.m[0] = self.m[0] * other.m[0] + self.m[1] * self.m[3] + self.m[2] * self.m[6];
        self.m[1] = self.m[0] * other.m[1] + self.m[1] * self.m[4] + self.m[2] * self.m[7];
        self.m[2] = self.m[0] * other.m[2] + self.m[1] * self.m[5] + self.m[2] * self.m[8];

        self.m[3] = self.m[3] * other.m[0] + self.m[4] * self.m[3] + self.m[5] * self.m[6];
        self.m[4] = self.m[3] * other.m[1] + self.m[4] * self.m[4] + self.m[5] * self.m[7];
        self.m[5] = self.m[3] * other.m[2] + self.m[4] * self.m[5] + self.m[5] * self.m[8];

        self.m[6] = self.m[6] * other.m[0] + self.m[7] * self.m[3] + self.m[8] * self.m[6];
        self.m[7] = self.m[6] * other.m[1] + self.m[7] * self.m[4] + self.m[8] * self.m[7];
        self.m[8] = self.m[6] * other.m[2] + self.m[7] * self.m[5] + self.m[8] * self.m[8];
    }
}

