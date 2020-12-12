use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Index, IndexMut, Neg};


pub type Color = Vec3;
pub type Point3 = Vec3;

pub struct Vec3 {
    coord: [f64; 3]
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub const fn new(c_0: f64, c_1: f64, c_2: f64) -> Self {
        Self {
            coord: [c_0, c_1, c_2]
        }
    }

    pub fn x(&self) -> f64 {
        self.coord[0]
    }
    pub fn r(&self) -> f64 {
        self.x()
    }

    pub fn y(&self) -> f64 {
        self.coord[1]
    }
    pub fn g(&self) -> f64 {
        self.y()
    }

    pub fn z(&self) -> f64 {
        self.coord[2]
    }
    pub fn b(&self) -> f64 {
        self.z()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        )
    }

    pub fn len(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn unit(&self) -> Self {
        let l = self.len();
        Self::new(
            self.x() / l,
            self.y() / l,
            self.z() / l
        )
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Self::new(
            self.x() + rhs,
            self.y() + rhs,
            self.z() + rhs
        )
    }
}

impl Add<&Self> for Vec3 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        Self::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z()
        )
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.coord[0] += rhs;
        self.coord[1] += rhs;
        self.coord[2] += rhs;
    }
}

impl AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.coord[0] += rhs.x();
        self.coord[1] += rhs.y();
        self.coord[2] += rhs.z();
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Self::new(
            self.x() - rhs,
            self.y() - rhs,
            self.z() - rhs
        )
    }
}

impl Sub<&Self> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        Self::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z()
        )
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.coord[0] -= rhs;
        self.coord[1] -= rhs;
        self.coord[2] -= rhs;
    }
}

impl SubAssign<&Self> for Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.coord[0] -= rhs.x();
        self.coord[1] -= rhs.y();
        self.coord[2] -= rhs.z();
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs
        )
    }
}

impl Mul<&Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        Self::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z()
        )
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.coord[0] *= rhs;
        self.coord[1] *= rhs;
        self.coord[2] *= rhs;
    }
}

impl MulAssign<&Self> for Vec3 {
    fn mul_assign(&mut self, rhs: &Self) {
        self.coord[0] *= rhs.x();
        self.coord[1] *= rhs.y();
        self.coord[2] *= rhs.z();
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs
        )
    }
}

impl Div<&Self> for Vec3 {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        Self::new(
            self.x() / rhs.x(),
            self.y() / rhs.y(),
            self.z() / rhs.z()
        )
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.coord[0] /= rhs;
        self.coord[1] /= rhs;
        self.coord[2] /= rhs;
    }
}

impl DivAssign<&Self> for Vec3 {
    fn div_assign(&mut self, rhs: &Self) {
        self.coord[0] /= rhs.x();
        self.coord[1] /= rhs.y();
        self.coord[2] /= rhs.z();
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.coord.len() {
            panic!("Index out of bounds {} / {}.", idx, self.coord.len());
        }
        else {
            &self.coord[idx]
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx >= self.coord.len() {
            panic!("Index out of bounds {} / {}.", idx, self.coord.len());
        }
        else {
            &mut self.coord[idx]
        }
    }
}

