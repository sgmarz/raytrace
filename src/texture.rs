// texture.rs
// Texturing functions
// Stephen Marz
// 15 Dec 2020

use crate::vector::{Vec3, Color};

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

#[derive(Default)]
pub struct SolidColor {
    color_value: Color
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self {
            color_value
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b)
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.color_value
    }
}

