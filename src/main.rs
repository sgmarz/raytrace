
pub mod picture;
pub mod ray;
pub mod ppm;
pub mod bmp;
pub mod vector;
pub mod matrix;
pub mod camera;
pub mod threadpool;

use crate::vector::Vec3;
use crate::picture::Picture;
use crate::bmp::BmpPicture;
use crate::camera::Camera;
use std::sync::Arc;

use std::env;

const NUM_THREADS: usize = 10;

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

    let mut tp = threadpool::ThreadPool::new(NUM_THREADS);
    let ih = image_height as f64 - 1.0;
    let iw = image_width as f64 -1.0;
    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / iw;
            let v = j as f64 / ih;
            tp.run_c(i, j, u, v, camera.clone());
        }
        eprint!("\rTraced row {:4} / {:4}", j+1, image_height);
    }
    println!("...done!                    ");

    for t in tp.threads.drain(..) {
        for _ in 0..t.packets_sent {
            let dpacket = t.data.recv().unwrap();
            pictwrite.set_pixel(dpacket.row, dpacket.col, &dpacket.color);
        }
    }

    println!("Writing bitmap file: '{}'", filename);
    if let Ok(px) = pictwrite.write_file(filename) {
        println!("Wrote {} bytes to {}.", px, filename);
    }
    else {
        println!("Unable to write to file.");
    }

}


