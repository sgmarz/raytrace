use crate::camera::Camera;
use crate::hitable::HitList;
use crate::random::random_f64;
use crate::vector::Vec3;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use std::vec::Vec;

pub struct ControlPacket {
	pub row: u32,
	pub col: u32,
	pub camera: Arc<Camera>,
	pub objects: Arc<HitList>,
	pub done: bool,
	pub samples: u32,
	pub image_width: u32,
	pub image_height: u32,
	pub max_depth: i32,
}

impl ControlPacket {
	pub const fn new(row: u32, col: u32, camera: Arc<Camera>, objects: Arc<HitList>, samples: u32, image_width: u32, image_height: u32, max_depth: i32) -> Self {
		Self {
			row,
			col,
			camera,
			objects,
			done: false,
			samples,
			image_width,
			image_height,
			max_depth,
		}
	}
	pub fn done() -> Self {
		Self {
			row: 0,
			col: 0,
			camera: Arc::new(Camera::default()),
			objects: Arc::new(HitList::default()),
			done: true,
			samples: 0,
			image_width: 0,
			image_height: 0,
			max_depth: 0,
		}
	}
}

pub struct DataPacket {
	pub row: u32,
	pub col: u32,
	pub color: Vec3,
}

impl DataPacket {
	pub const fn new(row: u32, col: u32, color: Vec3) -> Self {
		Self {
			row,
			col,
			color,
		}
	}
}

pub struct Thread {
	pub thread: JoinHandle<()>,
	pub data: Receiver<DataPacket>,
	pub control: Sender<ControlPacket>,
	pub packets_sent: usize,
}

pub struct ThreadPool {
	pub threads: Vec<Thread>,
	pub next: usize,
}

impl ThreadPool {
	pub fn new(num_threads: usize) -> Self {
		assert!(num_threads > 0);
		let mut threads = Vec::with_capacity(num_threads);
		let background = Vec3::new(0.0, 0.0, 0.0);
		for _ in 0..num_threads {
			let (data_s, data_r): (Sender<DataPacket>, Receiver<_>) = channel();
			let (control_s, control_r): (Sender<ControlPacket>, Receiver<_>) = channel();
			let cws = data_s.clone();
			let t = Thread {
				thread: spawn(move || {
					while let Ok(packet) = control_r.recv() {
						if packet.done == true {
							break;
						}
						let iwf = packet.image_width as f64 - 1.0;
						let ihf = packet.image_height as f64 - 1.0;
						let mut color = Vec3::new(0.0, 0.0, 0.0);
						for _ in 0..packet.samples {
							let u = (random_f64() + packet.col as f64) / iwf;
							let v = (random_f64() + packet.row as f64) / ihf;
							color += &packet.camera.get_ray(u, v).color(&background, &packet.objects, 10);
						}
						let dp = DataPacket::new(packet.row, packet.col, color);

						cws.send(dp).unwrap();
					}
				}),
				data: data_r,
				control: control_s,
				packets_sent: 0,
			};
			threads.push(t);
		}
		Self {
			threads,
			next: 0,
		}
	}
	pub fn run(&mut self, control: ControlPacket) -> bool {
		let t = &mut self.threads[self.next];
		t.packets_sent += 1;
		let res = t.control.send(control);
		self.next += 1;
		if self.next >= self.threads.len() {
			self.next = 0;
		}
		res.is_ok()
	}

	pub fn run_c(&mut self, row: u32, col: u32, camera: Arc<Camera>, objects: Arc<HitList>, samples: u32, image_width: u32, image_height: u32, max_depth: i32) -> bool {
		let cp = ControlPacket::new(row, col, camera, objects, samples, image_width, image_height, max_depth);
		self.run(cp)
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		for t in self.threads.drain(..) {
			t.control.send(ControlPacket::done()).unwrap();
			t.thread.join().unwrap();
		}
	}
}
