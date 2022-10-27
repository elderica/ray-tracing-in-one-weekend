use rand::{self, Rng};
use std::{
    cell::{Ref, RefCell},
    io::{self, BufWriter, Write},
    rc::Rc,
};

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod rtweekend;
mod vec3;

fn ray_color(r: &ray::Ray, world: Ref<dyn hittable::HitTable>, depth: i32) -> vec3::Color {
    if depth <= 0 {
        return vec3::Color::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, rtweekend::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material().scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return vec3::Color::zero();
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
    let material_ground: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(vec3::Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn material::Material> =
        Rc::new(material::Lambertian::new(vec3::Color::new(0.7, 0.3, 0.3)));
    let material_left: Rc<dyn material::Material> =
        Rc::new(material::Metal::new(vec3::Color::new(0.8, 0.8, 0.8)));
    let material_right: Rc<dyn material::Material> =
        Rc::new(material::Metal::new(vec3::Color::new(0.8, 0.6, 0.2)));

    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));
    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_center),
    )));
    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    )));
    world.borrow_mut().add(Rc::new(hittable::Sphere::new(
        vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
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
