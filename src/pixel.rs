
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red, green, blue
        }
    }
    pub fn set_red(&mut self, val: u8) {
        self.red = val;
    }
    pub fn set_green(&mut self, val: u8) {
        self.green = val;
    }
    pub fn set_blue(&mut self, val: u8) {
        self.blue = val;
    }
    pub fn get_red(&self) -> u8 {
        self.red
    }
    pub fn get_green(&self) -> u8 {
        self.green
    }
    pub fn get_blue(&self) -> u8 {
        self.blue
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}
