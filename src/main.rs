use std::{
    cell::{Ref, RefCell},
    io::{self, BufWriter, Write},
    rc::Rc,
};

mod color;
mod hittable;
mod ray;
mod rtweekend;
mod vec3;

fn vec3_to_color(v: vec3::Vec3) -> color::Color {
    color::Color::new(v.x(), v.y(), v.z())
}

fn ray_color(r: &ray::Ray, world: Ref<dyn hittable::HitTable>) -> color::Color {
    if let Some(rec) = world.hit(r, 0.0, rtweekend::INFINITY) {
        let v = 0.5 * (rec.normal() + vec3::Vec3::new(1.0, 1.0, 1.0));
        return vec3_to_color(v);
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    let v = (1.0 - t) * vec3::Vec3::new(1.0, 1.0, 1.0) + t * vec3::Vec3::new(0.5, 0.7, 1.0);
    vec3_to_color(v)
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    let world = Rc::new(RefCell::new(hittable::HitTableList::default()));
    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: rtweekend::Point3 = vec3::Vec3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - vec3::Vec3::new(0.0, 0.0, focal_length);

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
        writeln!(buferr, "Scan lines remaining:{}", j)?;
        for i in 0..image_width {
            let u = i as f64 / ((image_width - 1) as f64);
            let v = j as f64 / (image_height - 1) as f64;
            let d = lower_left_corner + (u * horizontal) + (v * vertical) - origin;
            let r = ray::Ray::new(origin, d);
            let pixel_color = ray_color(&r, world.borrow());
            writeln!(bufout, "{}", pixel_color)?;
        }
    }
    writeln!(buferr, "Done.")?;

    Ok(())
}
