
pub mod picture;
pub mod ray;
pub mod ppm;
pub mod bmp;
pub mod vector;
pub mod matrix;
pub mod camera;

use crate::vector::Vec3;
use crate::picture::Picture;
use crate::bmp::BmpPicture;
use crate::camera::Camera;
use std::sync::Arc;

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

    let camera = Arc::new(Camera::new(&Vec3::default(), viewport_width, viewport_height));
    let mut pictwrite = BmpPicture::new(image_width, image_height);
    let pictptr = &mut pictwrite as *mut BmpPicture as usize;

    const MAX_THREADS: usize = 15;
    let mut threads = Vec::<JoinHandle<()>>::with_capacity(MAX_THREADS);
    for j in 0..image_height {
        print!("\rTracing row {:4} / {:4}", j, image_height);
        for i in 0..image_width {
            let myi = i;
            let myj = j;
            let iw = image_width;
            let ih = image_height;
            let cref = camera.clone();
            while threads.len() > MAX_THREADS {
                threads.pop().unwrap().join().unwrap();
            }
            let h = spawn(move || {
                let u = myi as f64 / (iw as f64 - 1.0);
                let v = myj as f64 / (ih as f64 - 1.0);
                let color = cref.ray(u, v).trace();

                // This is a stupid way to get a threaded mutable reference. Since
                // i and j are with the thread, we cannot write to the same pixel with different
                // threads, meaning that this is thread safe.
                let p = pictptr as *mut BmpPicture;
                unsafe {
                    (*p).set_pixel(i, j, &color);
                }
            });
            threads.push(h);
        }
        for t in threads.drain(..) {
            t.join().unwrap();
        }
    }
    println!("done!                    ");

    if let Ok(px) = pictwrite.write_file(filename) {
        println!("Wrote {} bytes to {}.", px, filename);
    }
    else {
        println!("Unable to write to file.");
    }

}


