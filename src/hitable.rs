
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::material::Material;
use std::boxed::Box;
use std::vec::Vec;


pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord<'a> {
    pub fn new(point: Vec3, normal: Vec3, material: &'a dyn Material, t: f64) -> Self {
        Self {
            point,
            normal,
            material,
            t,
            front_face: false
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList {
    objects: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: &dyn Hitable) {
        self.objects.push(Box::new(object));
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        None
    }
}





