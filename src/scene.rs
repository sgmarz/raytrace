
use crate::surface::HitSurface;
use std::vec::Vec;
use std::boxed::Box;

pub struct Scene {
    pub surfaces: Vec<Box<dyn HitSurface>>
}
