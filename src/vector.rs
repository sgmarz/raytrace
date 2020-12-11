use std::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign, Index, IndexMut, Sub, SubAssign};
use crate::matrix::Mat3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn new_scalar(v: f64) -> Self {
        Self {
            x: v,
            y: v,
            z: v
        }
    }
    
    pub fn mul_scalar(&self, v: f64) -> Self {
        Self {
            x: self["x"] * v,
            y: self["y"] * v,
            z: self["z"] * v
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z    
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let l = self.len();
        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l
        }
    }

    pub fn clamp(&self, val_low: f64, val_high: f64) -> Self {
        let x = if self.x < val_low { val_low } else if self.x > val_high { val_high } else { self.x };
        let y = if self.y < val_low { val_low } else if self.y > val_high { val_high } else { self.y };
        let z = if self.z < val_low { val_low } else if self.z > val_high { val_high } else { self.z };
        Self {
            x,
            y,
            z
        }
    }

    pub fn mul_mat(&self, mat: &Mat3) -> Self {
        Self {
            x: self.x * mat[0] + self.y * mat[1] + self.z * mat[2],
            y: self.x * mat[3] + self.y * mat[4] + self.z * mat[5],
            z: self.x * mat[6] + self.y * mat[7] + self.z * mat[8]
        }
    }

    pub fn radd(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn rsub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn rmul(&self, other: &Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn rdiv(&self, other: &Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector index out of bounds: {} / 3", index)
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vector index out of bounds: {} / 3", index)
        }
    }
}

impl Index<&str> for Vec3 {
    type Output = f64;
    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "x" | "r" | "red" => &self.x,
            "y" | "g" | "green"=> &self.y,
            "z" | "b" | "blue" => &self.z,
            _ => panic!("Vector index out of bounds: {} / 3", index)
        }
    }
}

impl IndexMut<&str> for Vec3 {
    fn index_mut(&mut self, index: &str) -> &mut f64 {
        match index {
            "x" | "r" | "red" => &mut self.x,
            "y" | "g" | "green"=> &mut self.y,
            "z" | "b" | "blue" => &mut self.z,
            _ => panic!("Vector index out of bounds: {} / 3", index)
        }
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
