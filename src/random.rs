
use crate::vector::Vec3;
use crate::hitable::HitList;
use crate::material::Material;
use crate::objects::sphere::Sphere;
use std::sync::Arc;

use rand::Rng;

pub fn random_vector() -> Vec3 {
    let mut r = rand::thread_rng();
    let x = r.gen_range(-1.0, 1.0);
    let y = r.gen_range(-1.0, 1.0);
    let z = r.gen_range(-1.0, 1.0);
    Vec3::new(x, y, z)
}

pub fn random_f64() -> f64 {
    let mut r = rand::thread_rng();
    r.gen_range(0.0, 1.0)
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut r = rand::thread_rng();
    r.gen_range(min, max)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vector();
        if p.len2() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

pub fn random_scene() -> HitList {
    let mut world = HitList::new();

    let ground_material = Material::new_lambertian(Vec3::new(0.5, 0.5, 0.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if (center - &Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::new(random_f64(), random_f64(), random_f64()) * &Vec3::new(random_f64(), random_f64(), random_f64());
                    let sphere_material = Material::new_lambertian(albedo);
                    world.add(Arc::new(Sphere::new(center.clone(), 0.2, sphere_material)));
                } 
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::new(random_f64(), random_f64(), random_f64());
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Material::new_metal(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center.clone(), 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Material::new_dielectric(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Material::new_dielectric(1.5);
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Material::new_lambertian(Vec3::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Material::new_metal(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}
