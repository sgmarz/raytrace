
pub mod pict;
pub mod ray;
pub mod ppm;
pub mod bmp;
pub mod vector;
pub mod matrix;
pub mod camera;

use crate::vector::{Point3, Vec3};
use crate::ray::Ray;
use crate::bmp::BmpPicture;
use crate::pict::Picture;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [filename]", &args[0]);
        return;
    }

    let filename = &args[1];

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let v2 = Vec3::new_scalar(2.0);

    let f0 = horizontal.rdiv(&v2);
    let f1 = vertical.rdiv(&v2);
    let p01 = origin.rsub(&f0).rsub(&f1);
    let lower_left_corner = p01 - Vec3::new(0.0, 0.0, focal_length);

    let mut ppm = BmpPicture::new(image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = Vec3::new_scalar(i as f64 / (image_width as f64 - 1.0));
            let v = Vec3::new_scalar(j as f64 / (image_height as f64 - 1.0));
            let f0 = u.rmul(&horizontal);
            let f1 = v.rmul(&vertical);
            let p01 = lower_left_corner.radd(&f0.radd(&f1));
            let d = p01.rsub(&origin);
            let pixel_color = Ray::new(&origin, &d).trace();
            ppm.set_pixel(i, j, &pixel_color);
        }
    }

    if let Ok(px) = ppm.write_file(filename) {
        println!("Wrote {} bytes to {}.", px, filename);
    }
    else {
        println!("Unable to write to file.");
    }

}


