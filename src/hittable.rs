use crate::ray::Ray;
use crate::rtweekend::Point3;
use crate::vec3::{dot, Vec3};
use std::ops::Neg;
use std::rc::Rc;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    // front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64) -> Self {
        // let front_face = false;
        let normal = Vec3::zero();
        Self {
            p,
            normal,
            t,
            // front_face,
        }
    }

    pub fn set_face_normal(&self, r: &Ray, outward_normal: &Vec3) -> Self {
        let p = self.p;
        let t = self.t;
        let front_face = dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            outward_normal.neg()
        };
        Self {
            p,
            normal,
            t,
            // front_face,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait HitTable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl HitTable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius() * self.radius();

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - *self.center()) / self.radius();
        let h = HitRecord::new(p, t).set_face_normal(r, &outward_normal);

        Some(h)
    }
}

#[derive(Default)]
pub struct HitTableList {
    objects: Vec<Rc<dyn HitTable>>,
}

impl HitTableList {
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, object: Rc<dyn HitTable>) {
        self.objects.push(Rc::clone(&object))
    }
}

impl HitTable for HitTableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(h) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = h.t();
                temp_rec = Some(h);
            }
        }

        temp_rec
    }
}