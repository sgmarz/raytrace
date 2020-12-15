// main.rs
// Entry point for raytracer
// Stephen Marz
// 9 Dec 2020

pub mod bmp;
pub mod bounding_box;
pub mod bvh;
pub mod camera;
pub mod hitable;
pub mod material;
pub mod objects;
pub mod random;
pub mod ray;
pub mod texture;
pub mod threadpool;
pub mod vector;

use crate::{camera::Camera, random::random_scene, vector::Vec3};
use std::{env::args, sync::Arc};

const DEFAULT_PIXELS_UPDATE: i32 = 1000;

fn main() {
	let args: Vec<String> = args().collect();
	if args.len() < 7 {
		println!("Usage: {} [filename] [width] [height] [samples] [max depth] [num threads] <progress update interval>", args[0]);
		return;
	}

	let filename = &args[1];
	let image_width = args[2].parse::<u32>().unwrap();
	let image_height = args[3].parse::<u32>().unwrap();
	let aspect_ratio = image_width as f64 / image_height as f64;
	let samples = args[4].parse::<u32>().unwrap();
	let max_depth = args[5].parse::<i32>().unwrap();
	let num_threads = args[6].parse::<usize>().unwrap();
	let mut pixel_update = DEFAULT_PIXELS_UPDATE;

	if args.len() >= 8 {
		pixel_update = args[7].parse::<i32>().unwrap();
	}

	// Set up the camera parameters
	let lookfrom = Vec3::new(13.0, 2.0, 3.0);
	let lookat = Vec3::new(0.0, 0.0, 0.0);
	let vup = Vec3::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.01;
	let vfov = 20.0;
	let time0 = 0.0;
	let time1 = 1.0;

	// Create the camera, world, thread pool, and picture writer (to BMP for now)
	let camera = Arc::new(Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, time0, time1));
	let world = Arc::new(random_scene());
	let mut pool = threadpool::ThreadPool::new(num_threads);
	let mut pictwriter = bmp::BmpPicture::new(image_width, image_height, samples);

	eprintln!("Scene created, spawning threads.");
	// Spawn the thread pool with the work that needs to be done.
	for j in 0..image_height {
		for i in 0..image_width {
			pool.run_c(j, i, camera.clone(), world.clone(), samples, image_width, image_height, max_depth);
		}
	}
	// Even though we get here, the work the threads are doing isn't necessarily done.
	eprintln!("Threads spawned, working to render {}x{} image.", image_width, image_height);
	eprintln!(
		"Updating progress every {} pixel{}.",
		pixel_update,
		if pixel_update == 1 {
			""
		} else {
			"s"
		}
	);
	let mut pixels_remaining = 0;
	let mut pixels_written = 0;
	let total_pixels = image_width * image_height;

	// We get the data from the threads. Recv may block here, which might prevent
	// us from getting data from another thread, however, the work has to get done
	// anyway before we write to the BMP file.
	for t in pool.threads.drain(..) {
		for _ in 0..t.packets_sent {
			let d = t.data.recv().unwrap();
			pictwriter.set_pixel(d.col, d.row, &d.color);

			if pixels_remaining <= 0 {
				eprint!("\r{:10}/{:<10} pixels traced.", pixels_written, total_pixels);
				pixels_remaining = pixel_update;
			}
			pixels_remaining -= 1;
			pixels_written += 1;
		}
	}
	eprintln!("\r{:10}/{:<10} pixels traced.", pixels_written, total_pixels);
	eprintln!("Done writing pixels, writing to BMP file.");

	// When we get here, the pixels have been traced and written in memory. Now,
	// write them to the BMP file.
	if let Ok(_) = pictwriter.write_file(filename) {
		println!("Wrote to file '{}'", filename);
	} else {
		println!("Unable to write to file '{}'", filename);
	}
}
