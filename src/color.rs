use crate::vec3::*;
use std::fmt::Display;
pub struct Color(pub Vec3);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.0.x()) as usize,
            (255.999 * self.0.y()) as usize,
            (255.999 * self.0.z()) as usize
        )
    }
}
