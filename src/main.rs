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
pub mod perlin;
pub mod png;
pub mod random;
pub mod ray;
pub mod texture;
pub mod threadpool;
pub mod vector;

use crate::hitable::HitList;
use crate::material::Material;
use crate::objects::sphere::Sphere;
use crate::objects::xyrect::XyRect;
use crate::random::random_double;
use crate::texture::CheckeredTexture;
use crate::texture::ImageTexture;
use crate::texture::SolidColor;
use crate::threadpool::ThreadPool;
use crate::{camera::Camera, vector::Vec3};
use std::{env::args, sync::Arc};

const DEFAULT_PIXELS_UPDATE: i32 = 1000;

fn main() {
	let args: Vec<String> = args().collect();
	if args.len() < 7 {
		println!("Usage: {} [filename] [width] [height] [samples] [max depth] [num threads] <frames> <progress update interval>", args[0]);
		return;
	}

	let filename = &args[1];
	let image_width = args[2].parse::<u32>().unwrap();
	let image_height = args[3].parse::<u32>().unwrap();
	let aspect_ratio = image_width as f64 / image_height as f64;
	let samples = args[4].parse::<u32>().unwrap();
	let max_depth = args[5].parse::<i32>().unwrap();
	let num_threads = args[6].parse::<usize>().unwrap();
	let mut frames = 1usize;
	let mut pixel_update = DEFAULT_PIXELS_UPDATE;

	if args.len() >= 8 {
		frames = args[7].parse::<usize>().unwrap();
	}

	if args.len() >= 9 {
		pixel_update = args[8].parse::<i32>().unwrap();
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

	let mut spheres = Vec::<Sphere>::with_capacity(25);
	let checker = Arc::new(CheckeredTexture::new_color(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
	spheres.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::new_lambertian(checker)));

	for _ in 0..1 {
		// let solid = Arc::new(SolidColor::from_rgb(random_double(0.0, 1.0), random_double(0.0, 1.0), random_double(0.0, 1.0)));
		let texture = Arc::new(ImageTexture::from_file("d.png"));
		let material = Material::new_lambertian(texture);
		// let center = Vec3::new(random_double(-0.5, 1.5), random_double(0.2, 2.0), random_double(-2.0, 2.0));
		// let radius = random_double(0.07, 0.7);
		let center = Vec3::new(0.0, 0.0, 0.0);
		let radius = 2.0;
		let sphere = Sphere::new(center, radius, material);
		spheres.push(sphere)
	}

	// Create the camera, world, thread pool, and picture writer (to BMP for now)
	let camera = Arc::new(Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, time0, time1));
	// let mut scene = random_scene();
	for frame in 0..frames {
		let mut pool = threadpool::ThreadPool::new(num_threads);
		let mut frame_filename = String::from(filename);
		frame_filename.push('-');
		frame_filename.push_str(frame.to_string().as_str());
		frame_filename.push_str(".png");

		run(camera.clone(), make_world(&spheres), &mut pool, samples, image_width, image_height, max_depth, pixel_update, frame_filename.as_str());
		for i in 1..spheres.len() {
			let sphere = &mut spheres[i];
			let x = random_double(0.05, 0.3);
			// let y = random_double(0.2, 1.2);
			let z = random_double(-0.3, 0.3);
			let center = sphere.center_mut();
			center[0] += x;
			// center[1] += y;
			center[2] += z;
		}
	}
}

fn make_world(spheres: &Vec<Sphere>) -> Arc<HitList> {
	let mut world = HitList::new();

	let solid_white = SolidColor::from_rgb(4.0, 4.0, 4.0);
	let light_mat = Material::new_diffuse_light(Arc::new(solid_white));
	let rect = XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, light_mat);
	world.add(Arc::new(rect));

	for sphere in spheres.iter() {
		world.add(Arc::new(sphere.clone()));
	}

	Arc::new(world)
}

fn run(camera: Arc<Camera>, world: Arc<HitList>, pool: &mut ThreadPool, samples: u32, image_width: u32, image_height: u32, max_depth: i32, pixel_update: i32, filename: &str) {
	eprint!("Running '{}' Scene created, spawning threads....", filename);
	// Spawn the thread pool with the work that needs to be done.
	for j in 0..image_height {
		for i in 0..image_width {
			pool.run_c(j, i, camera.clone(), world.clone(), samples, image_width, image_height, max_depth);
		}
	}
	// Even though we get here, the work the threads are doing isn't necessarily done.
	eprintln!("done.\nWorking to render {}x{} image.", image_width, image_height);
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
	let mut pictwriter = png::PngPicture::new(image_width, image_height, samples);
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
