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

fn ray_color(r: &ray::Ray, world: Ref<dyn hittable::HitTable>, depth: i32) -> vec3::Color {
    if depth <= 0 {
        return vec3::Color::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, rtweekend::INFINITY) {
        let target: vec3::Point3 = rec.p() + rec.normal() + vec3::Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&ray::Ray::new(rec.p(), target - rec.p()), world, depth - 1);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

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
        write!(err, "\rScan lines remaining: {} ", j)?;
        err.flush()?;
        for i in 0..image_width {
            let mut pixel_color = vec3::Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(image_width - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, world.borrow(), max_depth);
            }
            color::write_color(&mut bufout, pixel_color, samples_per_pixel)?;
        }
    }
    writeln!(err, "\nDone.")?;

    Ok(())
}
