use rand::{self, Rng};
use rtweekend::PI;
use std::{
    io::{self, BufWriter, Write},
    rc::Rc,
};

mod material;
use material::{Dielectric, Lambertian, Material, Metal};

mod vec3;
use vec3::{Color, Point3};

mod hittable;
use hittable::{Hittable, HittableList, Sphere};

mod camera;
use camera::Camera;

mod ray;
use ray::Ray;

mod color;
mod rtweekend;

fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, rtweekend::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }

    let unit_direction = vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let r = f64::cos(PI / 4.0);
    let mut world = HittableList::default();
    let material_left: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        Rc::clone(&material_left),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        Rc::clone(&material_right),
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

    let cam = Camera::new(90.0, aspect_ratio);
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
                pixel_color += ray_color(&r, &world, max_depth);
            }
            color::write_color(&mut bufout, pixel_color, samples_per_pixel)?;
        }
    }
    writeln!(err, "\nDone.")?;

    Ok(())
}
