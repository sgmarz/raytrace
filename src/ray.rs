use crate::hitable::HitList;
use crate::vector::{Color, Vec3};
pub struct Ray {
	origin: Vec3,
	direction: Vec3,
	time: f64,
}

impl Ray {
	pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
		Self {
			origin,
			direction,
			time,
		}
	}

	pub fn origin(&self) -> &Vec3 {
		&self.origin
	}

	pub fn direction(&self) -> &Vec3 {
		&self.direction
	}

	pub fn time(&self) -> f64 {
		self.time
	}

	pub fn at(&self, t: f64) -> Vec3 {
		self.origin + &(self.direction * t)
	}

	pub fn color(&self, background: &Color, world: &HitList, depth: i32) -> Color {
		if depth <= 0 {
			Color::new(0.0, 0.0, 0.0)
        } 
        else if let Some(rec) = world.hit(self, 0.001, std::f64::INFINITY) {
            let emitted = rec.material().emitted(rec.u(), rec.v(), rec.point());
			if let Some((attenuation, scattered)) = rec.material().scatter(self, &rec) {
				emitted + &(attenuation * &scattered.color(background, world, depth - 1))
            } 
            else {
                emitted
			}
        } 
        else {
            *background
		}
	}
}
