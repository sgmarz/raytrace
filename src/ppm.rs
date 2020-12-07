use crate::pict::Picture;
use crate::pixel::Pixel;
use std::fs::File;
use std::io::{BufWriter, Error, Write};

pub struct Row {
    cols: Vec<Pixel>
}

pub struct PpmPicture {
    width: u32,
    height: u32,
    intensity: u32,
    rows: Vec<Row>
}

impl PpmPicture {
    pub fn new(width: u32, height: u32, intensity: u32) -> Self {
        Self {
            width,
            height,
            intensity,
            rows: Vec::new()
        }
    }
    pub fn write_file(&self, fname: &String) -> Result<(), Error> {
        let fc = File::create(fname);
        if let Ok(fl) = fc {
            let mut wd = BufWriter::new(fl);
            write!(wd, "P3\n{} {}\n{}\n", self.width, self.height, self.intensity)?;
            let mut i = 0;
            for row in self.rows.iter() {
                for pixel in row.cols.iter() {
                    write!(wd, "{} {} {}", pixel.red, pixel.green, pixel.blue)?;
                    i += 1;
                    if i > 10 {
                        write!(wd, "\n")?;
                        i = 0;
                    }
                    else {
                        write!(wd, " ")?;
                    }
                }
            }
            Ok(())
        }
        else {
            Err(fc.err().unwrap())
        }
    }
}

impl Picture for PpmPicture {
    fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let p = &self.rows[y as usize].cols[x as usize];
        Pixel {
            red: p.red,
            green: p.green,
            blue: p.blue
        }
    }
    fn set_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        let p = &mut self.rows[y as usize].cols[x as usize];
        p.red = pixel.red;
        p.green = pixel.green;
        p.blue = pixel.blue;
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
}
