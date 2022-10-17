use crate::rtweekend::clamp;
use std::{io, ops};

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

pub fn write_color<W: io::Write>(
    w: &mut W,
    pixel_color: Color,
    samples_per_pixel: u32,
) -> io::Result<()> {
    let r = pixel_color.r();
    let g = pixel_color.g();
    let b = pixel_color.b();

    let scale = 1.0 / f64::from(samples_per_pixel);
    let r: f64 = scale * r;
    let g: f64 = scale * g;
    let b: f64 = scale * b;

    let cr = (256.0 * clamp(r, 0.0, 0.999)) as usize;
    let cg = (256.0 * clamp(g, 0.0, 0.999)) as usize;
    let cb = (256.0 * clamp(b, 0.0, 0.999)) as usize;

    writeln!(w, "{} {} {}", cr, cg, cb)?;
    Ok(())
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
