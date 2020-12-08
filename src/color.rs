pub struct Color {
    red: f64,
    green: f64,
    blue: f64
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            red, green, blue
        }
    }
    pub fn get_red(&self) -> f64 {
        self.red
    }
    pub fn get_green(&self) -> f64 {
        self.green
    }
    pub fn get_blue(&self) -> f64 {
        self.blue
    }
    pub fn lerp(&self, t: f64) -> Color {
        Color::new((1.0 - t) * 1.0 + t * self.get_red(), (1.0 - t) * 1.0 + t * self.get_green(), (1.0 - t) * 1.0 + t * self.get_blue())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0
        }
    }
}
