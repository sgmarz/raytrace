
use crate::vector::{Color, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(o: &Vec3, d: &Vec3) -> Self {
        Self {
            origin: Vec3::new(o["x"], o["y"], o["z"]),
            direction: Vec3::new(d["x"], d["y"], d["z"])
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::new(self.origin["x"] + t * self.direction["x"],
                  self.origin["y"] + t * self.direction["y"],
                  self.origin["z"] + t * self.direction["z"]
            )
    }

    pub fn trace(&self) -> Color {
        let t = hit_sphere(&Vec3::new(0.0,0.0,-1.0), 0.5, self);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0,0.0,-1.0)).normalize();
            Vec3::new(0.5 * (n[0] + 1.0), 0.5 * (n[1] + 1.0), 0.5 * (n[2] + 1.0))
        }
        else {
            let dir = self.direction.normalize();
            let t = 0.5 * (dir["y"] + 1.0);
            let c0 = Color::new(1.0 - t, 1.0 - t, 1.0 - t);
            let c1 = Color::new(t * 0.5, t * 0.7, t * 1.0);
            c0 * c1
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt() ) / (2.0*a)
    }
}


