use rand::{self, Rng};
use std::{
    cell::{Ref, RefCell},
    io::{self, BufWriter, Write},
    rc::Rc,
};

mod camera;
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
    let samples_per_pixel = 100;

    let world = Rc::new(RefCell::new(hittable::HitTableList::default()));
    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut bufout = BufWriter::new(io::stdout().lock());
    let mut err = io::stderr().lock();

    writeln!(
        bufout,
        "P3
{} {}
255",
        image_width, image_height
    )?;

    let cam = camera::Camera::new();
    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        writeln!(err, "Scan lines remaining:{}", j)?;
        for i in 0..image_width {
            let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(image_width - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, world.borrow());
            }
            color::write_color(&mut bufout, pixel_color, samples_per_pixel)?;
        }
    }
    writeln!(err, "Done.")?;

    Ok(())
}
