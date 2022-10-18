use crate::rtweekend::clamp;
use crate::vec3::Color;
use std::io;

pub fn write_color<W: io::Write>(
    w: &mut W,
    pixel_color: Color,
    samples_per_pixel: u32,
) -> io::Result<()> {
    let r = pixel_color.r();
    let g = pixel_color.g();
    let b = pixel_color.b();

    let scale = 1.0 / f64::from(samples_per_pixel);
    let r = f64::sqrt(scale * r);
    let g = f64::sqrt(scale * g);
    let b = f64::sqrt(scale * b);

    let cr = (256.0 * clamp(r, 0.0, 0.999)) as usize;
    let cg = (256.0 * clamp(g, 0.0, 0.999)) as usize;
    let cb = (256.0 * clamp(b, 0.0, 0.999)) as usize;

    writeln!(w, "{} {} {}", cr, cg, cb)?;
    Ok(())
}
