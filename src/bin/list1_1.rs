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
            writeln!(bufout, "# r:{} g:{} b:{}", r, g, b)?;

            let ir = (255.999 * r) as usize;
            let ig = (255.999 * g) as usize;
            let ib = (255.999 * b) as usize;

            writeln!(bufout, "{} {} {}", ir, ig, ib)?;
        }
    }
    writeln!(buferr, "Done.")?;

    Ok(())
}
