
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vector::{Color, Vec3};
use std::ops::{Sub, Add, Mul, Neg};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = rec.normal().add(&crate::random_unit_vector());
        let scattered = Ray::new(rec.point().clone(), scatter_direction);
        Some((self.albedo.clone(), scattered))
    }
}


pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray.direction().unit(), rec.normal());
        let scattered = Ray::new(rec.point().clone(), reflected + &(crate::random_in_unit_sphere() * self.fuzz));
        let attenuation = self.albedo.clone();
        if scattered.direction().dot(rec.normal()) > 0.0 {
            Some((attenuation, scattered))
        }
        else {
            None
        }
    }
}

pub struct DiElectric {
    ir: f64
}

impl DiElectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir
        }
    }
}

impl Material for DiElectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face() { 1.0 / self.ir } else { self.ir };

        let unit_direction = ray.direction().unit();
        let refracted = refract(&unit_direction, rec.normal(), refraction_ratio);

        let scattered = Ray::new(rec.point().clone(), refracted);
        Some((attenuation, scattered))
    }
}


fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v.sub(&(n.clone() * v.dot(n) * 2.0));
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3{
    let cos_theta = uv.neg().dot(n).min(1.0);
    let r_out_perp = uv.add(&n.mul(cos_theta)) * etai_over_etat;
    let perp = (1.0 - r_out_perp.len2()).abs().sqrt();
    let r_out_parallel = n.mul(perp);
    r_out_perp + &r_out_parallel
}
