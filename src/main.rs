

pub mod vector;
pub mod bmp;

fn main() {
    let v = vector::Vec3::new(1.0, 2.0, 3.0);
    let u = vector::Vec3::new(2.0, 3.0, 4.0);
    let z = v + 100.0;

    println!("Vector: {} {} {}", z.x(), z.y(), z.z());
}