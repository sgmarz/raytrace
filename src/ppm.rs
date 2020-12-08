use crate::pict::Picture;
use crate::pixel::Pixel;
use std::fs::File;
use std::io::{BufWriter, Error, Write};

pub struct Row {
    cols: Vec<Pixel>
}

impl Default for Row {
    fn default() -> Self {
        Self {
            cols: Vec::new()
        }
    }
}

impl Row {
    pub fn resize_width(&mut self, new_width: usize) {
        self.cols.resize_with(new_width as usize, Default::default);
    }
}

pub struct PpmPicture {
    width: u32,
    height: u32,
    intensity: u32,
    rows: Vec<Row>
}

impl PpmPicture {
    pub fn new(width: u32, height: u32, intensity: u32) -> Self {
        let mut r = Self {
            width,
            height,
            intensity,
            rows: Vec::new()
        };
        r.resize(width, height);
        r
    }
    pub fn write_file(&self, fname: &str) -> Result<usize, Error> {
        let fc = File::create(fname);
        if let Ok(fl) = fc {
            let mut wd = BufWriter::new(fl);
            write!(wd, "P3\n{} {}\n{}\n", self.width, self.height, self.intensity)?;
            let mut i = 0;
            let mut num_pixels: usize = 0;
            for row in self.rows.iter() {
                for pixel in row.cols.iter() {
                    write!(wd, "{} {} {}", pixel.get_red(), pixel.get_green(), pixel.get_blue())?;
                    i += 1;
                    num_pixels += 1;
                    if i > 10 {
                        write!(wd, "\n")?;
                        i = 0;
                    }
                    else {
                        write!(wd, " ")?;
                    }
                }
            }
            write!(wd, "\n")?;
            Ok(num_pixels)
        }
        else {
            Err(fc.err().unwrap())
        }
    }
}

impl Picture for PpmPicture {
    fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let p = &self.rows[y as usize].cols[x as usize];
        Pixel::new(p.get_red(), p.get_green(), p.get_blue())
    }
    fn set_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        let p = &mut self.rows[y as usize].cols[x as usize];
        p.set_red(pixel.get_red());
        p.set_green(pixel.get_green());
        p.set_blue(pixel.get_blue());
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
        let resize_width = || { let mut r = Row::default(); r.resize_width(new_width as usize); r} ;
        self.rows.resize_with(new_height as usize, resize_width);
    }
}
