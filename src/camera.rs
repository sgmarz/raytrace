
use crate::vector::Vec3;
use crate::ray::Ray;

#[derive(Default)]
pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub llc: Vec3
}

impl Camera {
    pub fn new(org: &Vec3, vp_width: f64, vp_height: f64) -> Self {
        let origin = Vec3::new(org[0], org[1], org[2]);
        let horizontal = Vec3::new(vp_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, vp_height, 0.0);
        const FOCAL_LENGTH: Vec3 = Vec3::new(0.0, 0.0, 1.0);
        const SCALAR_2: Vec3 = Vec3::new_scalar(2.0);
        let llc = org.rsub(&horizontal.rdiv(&SCALAR_2)).rsub(&vertical.rdiv(&SCALAR_2)).rsub(&FOCAL_LENGTH);
        Self {
            origin,
            horizontal,
            vertical,
            llc
        }
    }

    pub fn ray(&self, us: f64, vs: f64) -> Ray {
        let u = Vec3::new(us, us, us);
        let v = Vec3::new(vs, vs, vs);
        let direction = self.llc.radd(&self.horizontal.rmul(&u)).radd(&self.vertical.rmul(&v)).rsub(&self.origin);
        Ray::new(&self.origin, &direction)
    }
}

/*

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let v2 = Vec3::new_scalar(2.0);

    let f0 = horizontal.rdiv(&v2); // horizontal / 2
    let f1 = vertical.rdiv(&v2); // vertical / 2
    let p01 = origin.rsub(&f0).rsub(&f1); // origin - horizontal / 2 - vertical / 2
    let lower_left_corner = p01 - Vec3::new(0.0, 0.0, focal_length);

*/

