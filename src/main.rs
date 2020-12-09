
pub mod pict;
pub mod color;
pub mod ray;
pub mod ppm;
pub mod vector;
pub mod matrix;


fn main() {
    let c = color::Color::new(1.0, 2.0, 3.0);
    println!("Color: {}, {}, {}", c[0], c["green"], c["blue"]);
}
