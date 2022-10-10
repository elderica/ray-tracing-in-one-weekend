use ray_tracing_in_one_weekend::{unit_vector, Color, Point3, Ray, Vec3};
use std::io::{self, BufWriter, Write};

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    let v = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    Color::new(v.x(), v.y(), v.z())
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    let mut bufout = BufWriter::new(io::stdout().lock());
    let mut buferr = BufWriter::new(io::stderr().lock());

    writeln!(
        bufout,
        "P3
{} {}
255",
        image_width, image_height
    )?;

    for j in (0i32..image_height).rev() {
        writeln!(buferr, "Scanlines remaining:{}", j)?;
        for i in 0..image_width {
            let u = i as f64 / ((image_width - 1) as f64);
            let v = j as f64 / (image_height - 1) as f64;
            let d = lower_left_corner + (u * horizontal) + (v * vertical) - origin;
            let r = Ray::new(origin, d);
            let pixel_color = ray_color(&r);
            writeln!(bufout, "{}", pixel_color)?;
        }
    }
    writeln!(buferr, "Done.")?;

    Ok(())
}
