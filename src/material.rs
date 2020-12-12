
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vector::{Vec3, Color};
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = rand::thread_rng();
        let random_vector = Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)).normalize();
        let scatter_direction = rec.normal.radd(&random_vector);
        
    }
}

