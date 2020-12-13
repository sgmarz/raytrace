

pub mod vector;
pub mod bmp;
pub mod ray;
pub mod objects;
pub mod hitable;
pub mod material;
// pub mod threadpool;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::HitList;
use crate::material::Lambertian;
use crate::material::Material;
use std::rc::Rc;
use std::sync::Arc;
use std::ops::Add;
use std::env::args;

const IMAGE_WIDTH: u32 = 640;

use rand::Rng;

fn random_vector() -> Vec3 {
    let mut r = rand::thread_rng();
    let x = r.gen_range(-1.0, 1.0);
    let y = r.gen_range(-1.0, 1.0);
    let z = r.gen_range(-1.0, 1.0);
    Vec3::new(x, y, z)
}

fn random_f64() -> f64 {
    let mut r = rand::thread_rng();
    r.gen_range(0.0, 1.0)
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

fn ray_color(ray: &Ray, world: &HitList, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    else if let Some(rec) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some ((attenuation, scattered)) = rec.material().scatter(&ray, &rec) {
            return attenuation * &ray_color(&scattered, world, depth-1);
        }
        else {
            return Vec3::new(0.0,0.0,0.0);
        }
        // let target = rec.point().add(rec.normal()).add(&random_unit_vector());
        // ray_color(&Ray::new(rec.point().clone(), target - rec.point()), world, depth - 1) * 0.5
    }
    else {
        let unit_direction = ray.direction().unit();
        let t = (unit_direction.y() + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + &(Vec3::new(0.5, 0.7, 1.0) * t)
    }
}

fn main() {

    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Usage: {} [filename]", args[0]);
        return;
    }

    let filename = &args[1];

    let aspect_ratio = 16.0 / 9.0;
    let image_width = IMAGE_WIDTH;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let mut world = hitable::HitList::new();
    world.add(Rc::new(objects::sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.5, 0.25, 0.75))))));
    world.add(Rc::new(objects::sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.15, 0.20, 0.0))))));

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let focal_length_vector = Vec3::new(0.0, 0.0, focal_length);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - &(horizontal / 2.0) - &(vertical / 2.0) - &focal_length_vector;


    // let mut pool = threadpool::ThreadPool::new();
    let mut pictwriter = bmp::BmpPicture::new(IMAGE_WIDTH, image_height);

    let iwf = image_width as f64 - 1.0;
    let ihf = image_height as f64 - 1.0;

    for j in 0..image_height {
        eprint!("\rRow {:4} of {:4}", j+1, image_height);
        for i in 0..image_width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..10 {
                let u = (random_f64() + i as f64) / iwf;
                let v = (random_f64() + j as f64) / ihf;
                let r = Ray::new(origin, lower_left_corner + &(horizontal * u) + &(vertical * v));
                color += &ray_color(&r, &world, 40);
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