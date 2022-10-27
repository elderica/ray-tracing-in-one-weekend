pub const INFINITY: f64 = f64::INFINITY;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        x if x < min => min,
        x if x > max => max,
        x => x,
    }
}
