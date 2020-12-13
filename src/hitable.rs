
use crate::vector::Vec3;
use crate::ray::Ray;
use std::rc::Rc;
use std::vec::Vec;
use crate::material::{Material, Lambertian};

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f64,
    material: Rc<dyn Material>,
    front_face: bool
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false, Rc::new(Lambertian::new(Vec3::new(0.5, 0.25, 0.75))))
    }
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool, material: Rc<dyn Material>) -> Self {
        Self {
            point,
            normal,
            t,
            material,
            front_face,
        }
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }

    pub fn point(&self) -> &Vec3 {
        &self.point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitList {
    objects: Vec<Rc<dyn Hitable>>
}

impl HitList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, obj: Rc<dyn Hitable>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = rec.t();
                closest_rec = rec;
            }
        }

        if hit_anything {
            Some(closest_rec)
        }
        else {
            None
        }
    }
}

