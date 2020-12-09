
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

    pub fn conv_red(&self, intensity: u8) -> u8 {
        (self.red * intensity as f64) as u8
    }

    pub fn conv_green(&self, intensity: u8) -> u8 {
        (self.green * intensity as f64) as u8
    }

    pub fn conv_blue(&self, intensity: u8) -> u8 {
        (self.blue * intensity as f64) as u8
    }

    pub fn lerp(&self, t: f64) -> Color {
        Color::new((1.0 - t) * 1.0 + t * self.red, (1.0 - t) * 1.0 + t * self.green, (1.0 - t) * 1.0 + t * self.blue)
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

impl Index<&str> for Color {
    type Output = f64;
    fn index(&self, idx: &str) -> &Self::Output {
        match idx {
            "red" => &self.red,
            "green" => &self.green,
            "blue" => &self.blue,
            _ => panic!("Invalid color index {}", idx)
        }
    }
}

impl IndexMut<&str> for Color {
    fn index_mut(&mut self, idx: &str) -> &mut Self::Output {
        match idx {
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => panic!("Invalid color index {}", idx)
        }
    }
}
