
use std::ops::{Index, IndexMut};

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red, green, blue
        }
    }
    pub fn set_red(&mut self, val: u8) {
        self.red = val;
    }
    pub fn set_green(&mut self, val: u8) {
        self.green = val;
    }
    pub fn set_blue(&mut self, val: u8) {
        self.blue = val;
    }
    pub fn get_red(&self) -> u8 {
        self.red
    }
    pub fn get_green(&self) -> u8 {
        self.green
    }
    pub fn get_blue(&self) -> u8 {
        self.blue
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}

impl Index<usize> for Pixel {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => panic!("Invalid pixel {}", idx)
        }
    }
}

impl IndexMut<usize> for Pixel {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            _ => panic!("Invalid pixel {}", idx)
        }
    }
}
