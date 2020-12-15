
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::random::random_double;

#[derive(Default)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperature: f64, focus_dist: f64, time0: f64, time1: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        
        let w = (look_from - &look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - &(horizontal / 2.0) - &(vertical / 2.0) - &(w * focus_dist);

        let lens_radius = aperature / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            time0,
            time1
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + &(self.v * rd.y());
        let timeoff = random_double(self.time0, self.time1);
        Ray::new(self.origin().clone() + &offset, self.lower_left_corner + &(self.horizontal * s) + &(self.vertical * t) - self.origin() - &offset, timeoff)
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_double(-1.0, 1.0), random_double(-1.0,1.0), 0.0);
        if p.len2() >= 1.0 {
            continue;
        } 
        return p;
    }
}


