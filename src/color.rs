
use std::ops::{Index, IndexMut};
pub struct Color {
    red: f64,
    green: f64,
    blue: f64
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            red, green, blue
        }
    }
    pub fn get_red(&self) -> f64 {
        self.red
    }
    pub fn get_green(&self) -> f64 {
        self.green
    }
    pub fn get_blue(&self) -> f64 {
        self.blue
    }
    pub fn lerp(&self, t: f64) -> Color {
        Color::new((1.0 - t) * 1.0 + t * self.get_red(), (1.0 - t) * 1.0 + t * self.get_green(), (1.0 - t) * 1.0 + t * self.get_blue())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0
        }
    }
}

impl Index<usize> for Color {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => panic!("Invalid color index {}", idx)
        }
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            _ => panic!("Invalid color index {}", idx)
        }
    }
}