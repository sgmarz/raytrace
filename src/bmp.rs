use crate::picture::Picture;
use std::vec::Vec;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use crate::vector::Color;
use std::mem::size_of;

pub struct Row {
    cols: Vec<Color>
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

pub struct BmpPicture {
    width: u32,
    height: u32,
    rows: Vec<Row>
}

#[repr(packed)]
struct BitmapFileHeader {
    pub signature: u16,
    pub size: u32,
    pub rsv1: u16,
    pub rsv2: u16,
    pub offset: u32
}

impl Default for BitmapFileHeader {
    fn default() -> Self {
        Self {
            signature: 0,
            size: 0,
            rsv1: 0,
            rsv2: 0,
            offset: 0
        }
    }
}

#[repr(packed)]
struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bpp: u16,
    pub compression: u32,
    pub bmp_size: u32,
    pub hres: i32,
    pub vres: i32,
    pub colors: u32,
    pub clrs_used: u32
}

impl Default for BitmapInfoHeader {
    fn default() -> Self {
        Self {
            size: 0,
            width: 0,
            height: 0,
            planes: 0,
            bpp: 0,
            compression: 0,
            bmp_size: 0,
            hres: 0,
            vres: 0,
            colors: 0,
            clrs_used: 0
        }
    }
}

#[repr(packed)]
#[derive(Default)]
struct BitmapHeaders {
    pub bfh: BitmapFileHeader,
    pub bih: BitmapInfoHeader,
}

impl BmpPicture {
    pub fn new(width: u32, height: u32) -> Self {
        let mut r = Self {
            width,
            height,
            rows: Vec::new()
        };
        r.resize(width, height);
        r
    }

    pub fn write_file(&self, fname: &str) -> Result<usize, Error> {
        let mut wd = BufWriter::new(File::create(fname)?);
        let row_bytes = self.width as usize * 3;
        let padding = if row_bytes % 4 != 0 { 4 - row_bytes % 4 } else { 0 };
        let mut buffer = { let mut v = Vec::<u8>::new(); v.resize_with(size_of::<BitmapHeaders>(), Default::default); v };
        let mut header = unsafe { (buffer.as_mut_ptr() as *mut BitmapHeaders).as_mut().unwrap() };
        let mut bytes_written = 0;

        header.bfh.signature = 0x4d_42;
        header.bfh.size = size_of::<BitmapFileHeader>() as u32;
        header.bfh.offset = header.bfh.size;

        header.bih.width = self.width as i32;
        header.bih.height = self.height as i32;
        header.bih.size = size_of::<BitmapInfoHeader>() as u32;
        header.bih.colors = 0;
        header.bih.bpp = 24;
        header.bih.compression = 0;
        header.bih.planes = 1;
        header.bih.bmp_size = 0;
        header.bih.clrs_used = 0;
        header.bih.vres = 1;
        header.bih.hres = 1;

        let bufsl = buffer.into_boxed_slice();
        wd.write_all(&bufsl)?;
        bytes_written += bufsl.len();
        for row in (0..self.height).rev() {
            print!("\r{:4} rows remaining.", row);
            for col in 0..self.width {
                let px = self.get_pixel(col, row);
                let r = (px["r"] * 255.0) as u8;
                let g = (px["g"] * 255.0) as u8;
                let b = (px["b"] * 255.0) as u8;
                wd.write_all(&[b, g, r])?;
                bytes_written += 3;
            }
            for _ in 0..padding {
                write!(wd, "\x00")?;
            }
        }
        println!();
        wd.flush()?;
        Ok(bytes_written)
    }
}

impl Picture for BmpPicture {
    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let p = &self.rows[y as usize].cols[x as usize];
        Color::new(p[0], p[1], p[2])
    }
    fn set_pixel(&mut self, x: u32, y: u32, pixel: &Color) {
        let p = &mut self.rows[y as usize].cols[x as usize];
        p[0] = pixel[0];
        p[1] = pixel[1];
        p[2] = pixel[2];
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
