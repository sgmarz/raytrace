
pub mod pict;
pub mod ray;
pub mod pixel;
pub mod ppm;

use ray::Vec3;

fn main() {
    let a = Vec3::new(2.0, -1.0, 1.0);
    let b = Vec3::new(0.0, 1.0, 4.0);
    let dp = ray::cross(&a, &b);
    println!("Dot product is {}, {}, {}", dp.x, dp.y, dp.z);
}
