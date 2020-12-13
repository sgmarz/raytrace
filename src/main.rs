

pub mod vector;
pub mod bmp;
pub mod ray;
pub mod objects;
pub mod hitable;
pub mod material;
pub mod camera;
pub mod threadpool;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::HitList;
use crate::objects::sphere::Sphere;
use crate::material::Material;
use crate::camera::Camera;
use std::sync::Arc;
// use std::ops::Add;
use std::env::args;

use rand::Rng;

fn random_vector() -> Vec3 {
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

pub fn ray_color(ray: &Ray, world: &HitList, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    else if let Some(rec) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some ((attenuation, scattered)) = rec.material().scatter(&ray, &rec) {
            return attenuation * &ray_color(&scattered, world, depth-1);
        }
        else {
            return Vec3::new(1.0,1.0,1.0);
        }
    }
    else {
        let unit_direction = ray.direction().unit();
        let t = (unit_direction.y() + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + &(Vec3::new(0.5, 0.7, 1.0) * t)
    }
}

fn random_scene() -> HitList {
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

fn main() {

    let args: Vec<String> = args().collect();

    if args.len() < 5 {
        println!("Usage: {} [filename] [width] [ratio] [samples] <num threads>", args[0]);
        return;
    }

    let filename = &args[1];
    let image_width = args[2].parse::<u32>().unwrap();
    let aspect_ratio = args[3].parse::<f64>().unwrap();
    let samples = args[4].parse::<u32>().unwrap();
    let mut num_threads = 5usize;

    if args.len() >= 6 {
        num_threads = args[5].parse::<usize>().unwrap();
    }

    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let max_depth = 100;
    
    let world = random_scene();
    
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);
    let mut pool = threadpool::ThreadPool::new(num_threads);
    let mut pictwriter = bmp::BmpPicture::new(image_width, image_height, samples);

    let iwf = image_width as f64 - 1.0;
    let ihf = image_height as f64 - 1.0;

    for j in 0..image_height {
        eprint!("\rRow {:4} of {:4}", j+1, image_height);
        for i in 0..image_width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (random_f64() + i as f64) / iwf;
                let v = (random_f64() + j as f64) / ihf;
                let r = camera.get_ray(u, v);
                color += &ray_color(&r, &world, max_depth);
            }
            pictwriter.set_pixel(i, j, &color);
        }
    }
    eprintln!();
    if let Ok(_) = pictwriter.write_file(filename) {
        println!("Wrote to file '{}'", filename);
    }
    else {
        println!("Unable to write to file '{}'", filename);
    }

}