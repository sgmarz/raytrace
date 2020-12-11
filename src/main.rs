
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

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 481u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let v2 = Vec3::new_scalar(2.0);

    let lower_left_corner = origin - horizontal / v2 - vertical / v2 - Vec3::new(0.0, 0.0, focal_length);

    let mut ppm = BmpPicture::new(image_width, image_height);

    for i in 0..image_height {
        for j in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let us = Vec3::new_scalar(u);
            let vs = Vec3::new_scalar(v);
            let d = lower_left_corner + us * horizontal + vs * vertical - origin;
            let r = Ray::new(&origin, &d);
            let pixel_color = r.trace();
            ppm.set_pixel(j, i, &pixel_color);
        }
    }

    if let Ok(px) = ppm.write_file("rayt.bmp") {
        println!("Wrote {} bytes to rayt.bmp.", px);
    }
    else {
        println!("Unable to write to file.");
    }

}
