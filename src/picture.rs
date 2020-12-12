
use crate::vector::Color;

pub trait Picture {
    fn get_pixel(&self, row: u32, col: u32) -> Color;
    fn set_pixel(&mut self, row: u32, col: u32, pixel: &Color);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn resize(&mut self, new_width: u32, new_height: u32);
}
