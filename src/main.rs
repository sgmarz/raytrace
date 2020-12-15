

pub mod vector;
pub mod bmp;
pub mod ray;
pub mod objects;
pub mod hitable;
pub mod material;
pub mod camera;
pub mod threadpool;
pub mod random;

use crate::vector::Vec3;
use crate::camera::Camera;
use crate::random::random_scene;
use std::sync::Arc;
use std::env::args;

const PIXELS_UPDATE: i32 = 1000;


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
    let mut num_threads = 1usize;

    if args.len() >= 6 {
        num_threads = args[5].parse::<usize>().unwrap();
    }

    let image_height = (image_width as f64 / aspect_ratio) as u32;
    
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Arc::new(Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus));
    let world = Arc::new(random_scene());
    let mut pool = threadpool::ThreadPool::new(num_threads);
    let mut pictwriter = bmp::BmpPicture::new(image_width, image_height, samples);

    eprintln!("Scene created, spawning threads.");
    for j in 0..image_height {
        for i in 0..image_width {
            pool.run_c(j, i, camera.clone(), world.clone(), samples, image_width, image_height);
        }
    }
    eprintln!("Threads spawned, working to render {}x{} image.", image_width, image_height);
    eprintln!("Updating progress every {} pixels.", PIXELS_UPDATE);
    let mut pixels_remaining = 0;
    let mut pixels_written = 0;
    let total_pixels = image_width * image_height;
    for t in pool.threads.drain(..) {
        for _ in 0..t.packets_sent {
            let d = t.data.recv().unwrap();
            pictwriter.set_pixel(d.col, d.row, &d.color);

            if pixels_remaining <= 0 {
                eprint!("\r{:10}/{:<10} pixels traced.", pixels_written, total_pixels);
                pixels_remaining = PIXELS_UPDATE;
            }
            pixels_remaining -= 1;
            pixels_written += 1;
        }
    }
    eprintln!("\r{:10}/{:<10} pixels traced.", pixels_written, total_pixels);
    eprintln!("Done writing pixels, writing to BMP file.");

    if let Ok(_) = pictwriter.write_file(filename) {
        println!("Wrote to file '{}'", filename);
    }
    else {
        println!("Unable to write to file '{}'", filename);
    }

}