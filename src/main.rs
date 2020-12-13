

pub mod vector;
pub mod bmp;
pub mod ray;
pub mod objects;
pub mod hitable;
pub mod material;
pub mod camera;
// pub mod threadpool;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::HitList;
use crate::objects::sphere::Sphere;
use crate::material::{Lambertian, Metal, DiElectric};
use crate::camera::Camera;
use std::rc::Rc;
// use std::sync::Arc;
// use std::ops::Add;
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
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(DiElectric::new(1.5));
    let material_right  = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    
    world.add(Rc::new(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Vec3::new( 0.0,    0.0, -1.0),   0.5, material_center)));
    world.add(Rc::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, material_left.clone())));
    world.add(Rc::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0), -0.45, material_left)));
    world.add(Rc::new(Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, material_right)));
    
    let camera = Camera::new(Vec3::new(-2.0,2.0,1.0), Vec3::new(0.0,0.0,-1.0), Vec3::new(0.0,1.0,0.0), 20.0, aspect_ratio);
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
                let r = camera.get_ray(u, v);
                color += &ray_color(&r, &world, 20);
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