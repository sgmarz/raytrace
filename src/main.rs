

pub mod vector;
pub mod bmp;
pub mod ray;
pub mod objects;
pub mod hitable;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::HitList;
use std::rc::Rc;
use std::ops::Add;

const IMAGE_WIDTH: u32 = 400;

use rand::Rng;

fn random_vector() -> Vec3 {
    let mut r = rand::thread_rng();
    let x = r.gen_range(0.0, 1.0);
    let y = r.gen_range(0.0, 1.0);
    let z = r.gen_range(0.0, 1.0);
    Vec3::new(x, y, z)
}

fn ray_color(ray: &Ray, world: &HitList) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.0, std::f64::INFINITY) {
        rec.normal().add(&Vec3::new(1.0,1.0,1.0)) * 0.5
    }
    else {
        let unit_direction = ray.direction().unit();
        let t = (unit_direction.y() + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + &(random_vector() * t)
    }
}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = IMAGE_WIDTH;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let mut world = hitable::HitList::new();
    world.add(Rc::new(objects::sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(objects::sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let focal_length_vector = Vec3::new(0.0, 0.0, focal_length);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - &(horizontal / 2.0) - &(vertical / 2.0) - &focal_length_vector;

    let mut pictwriter = bmp::BmpPicture::new(IMAGE_WIDTH, image_height);

    let iwf = image_width as f64 - 1.0;
    let ihf = image_height as f64 - 1.0;

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / iwf;
            let v = j as f64 / ihf;
            let r = Ray::new(origin, lower_left_corner + &(horizontal * u) + &(vertical * v));
            let color = ray_color(&r, &world);
            pictwriter.set_pixel(i, j, &color);
        }
    }

    pictwriter.write_file("rayt.bmp").expect("Unable to write to rayt.bmp file.");

}