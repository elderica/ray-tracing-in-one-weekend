use std::fmt::Display;

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    // pub fn r(&self) -> f64 {
    //     self.r
    // }

    // pub fn g(&self) -> f64 {
    //     self.g
    // }

    // pub fn b(&self) -> f64 {
    //     self.b
    // }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.r) as usize,
            (255.999 * self.g) as usize,
            (255.999 * self.b) as usize
        )
    }
}
