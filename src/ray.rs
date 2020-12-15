
use crate::vector::Vec3;
use crate::hitable::HitList;
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + &(self.direction * t)
    }

    pub fn color(&self, world: &HitList, depth: i32) -> Vec3 {
        if depth <= 0 {
            Vec3::new(0.0, 0.0, 0.0)
        }
        else if let Some(rec) = world.hit(&self, 0.001, std::f64::INFINITY) {
            if let Some ((attenuation, scattered)) = rec.material().scatter(&self, &rec) {
                return attenuation * &scattered.color(world, depth-1);
            }
            else {
                return Vec3::new(1.0,1.0,1.0);
            }
        }
        else {
            let unit_direction = self.direction().unit();
            let t = (unit_direction.y() + 1.0) * 0.5;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + &(Vec3::new(0.5, 0.75, 1.0) * t)
        }
    }
}
