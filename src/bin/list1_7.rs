use ray_tracing_in_one_weekend::{color, vec3};
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    let mut bufout = BufWriter::new(io::stdout().lock());
    let mut buferr = BufWriter::new(io::stderr().lock());
    writeln!(
        bufout,
        "P3
{} {}
255",
        IMAGE_WIDTH, IMAGE_HEIGHT
    )?;

    for j in (0..IMAGE_HEIGHT).rev() {
        writeln!(buferr, "Scanlines remaning:{}", j)?;
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.25;
            let v = vec3::Vec3::new(r, g, b);
            let pixel_color = color::Color(v);

            writeln!(bufout, "{}", pixel_color)?;
        }
    }
    writeln!(buferr, "Done.")?;

    Ok(())
}
