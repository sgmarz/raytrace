
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
use std::thread::{spawn, JoinHandle};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [filename]", &args[0]);
        return;
    }

    let filename = &args[1];

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let v2 = Vec3::new_scalar(2.0);

    let f0 = horizontal.rdiv(&v2); // horizontal / 2
    let f1 = vertical.rdiv(&v2); // vertical / 2
    let p01 = origin.rsub(&f0).rsub(&f1); // origin - horizontal / 2 - vertical / 2
    let lower_left_corner = p01 - Vec3::new(0.0, 0.0, focal_length);

    let mut ppm = BmpPicture::new(image_width, image_height);

    for j in 0..image_height {
        let mut threads = Vec::<JoinHandle<(u32, u32, Vec3)>>::with_capacity(image_width as usize);
        for i in 0..image_width {
            let myi = i;
            let myj = j;
            let iw = image_width;
            let ih = image_height;
            let h = spawn(move || {
                let u = Vec3::new_scalar(myi as f64 / (iw as f64 - 1.0));
                let v = Vec3::new_scalar(myj as f64 / (ih as f64 - 1.0));
                let f0 = u.rmul(&horizontal);
                let f1 = v.rmul(&vertical);
                let p01 = lower_left_corner.radd(&f0.radd(&f1));
                let d = p01.rsub(&origin);
                let pixel_color = Ray::new(&origin, &d).trace();
                (i, j, pixel_color)
            });
            threads.push(h);
        }
        for t in threads.into_iter() {
            let (row, col, cl) = t.join().unwrap();
            ppm.set_pixel(row, col, &cl);
        }
    }


    if let Ok(px) = ppm.write_file(filename) {
        println!("Wrote {} bytes to {}.", px, filename);
    }
    else {
        println!("Unable to write to file.");
    }

}


