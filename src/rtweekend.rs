use crate::vec3::Vec3;

pub type Point3 = Vec3;

pub const INFINITY: f64 = f64::INFINITY;
// pub const PI: f64 = std::f64::consts::PI;

// pub type Degrees = f64;
// pub type Radians = f64;

// pub fn degrees_to_radians(degrees: Degrees) -> Radians {
//     degrees * PI / 180.0
// }

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        x if x < min => min,
        x if x > max => max,
        x => x,
    }
}
