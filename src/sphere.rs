use crate::{hittable::{self, Hittable}, interval::Interval, vector::Vector};
use crate::vector::*;
use crate::ppm::*;

pub struct Sphere {
    c: Vector,
    r: f64,
    col: Color
}

impl Sphere {
    pub fn new(c: Vector, r: f64) -> Self {
        Sphere {c: c, r: r, col: Color(1.0, 1.0, 1.0)}
    }

    pub fn center(&self) -> Vector {
        self.c
    }

    pub fn radius(&self) -> f64 {
        self.r
    }

    pub fn set_col(&mut self, c: Color) -> Self {
        self.col = c;
        Sphere {c: self.c, r: self.r, col: self.col}
    }

}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, interval: &Interval, object: &mut hittable::HitRecord) -> bool {
        let oc = self.c - *ray.origin(); 
        let a = ray.dir().length_squared();
        let h = dot(ray.dir(), &oc);
        let c = oc.length_squared() - self.r * self.r;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 { return false; }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;

        if !interval.surrounds(&root) {
            root = (h + sqrt_d) / a;
            if !interval.surrounds(&root) { return false; }
        }

        object.t = root;
        object.p = ray.at(object.t);
        // object.normal = (object.p - self.c) / self.r;
        let outward_normal: Vector = (object.p - self.c) / self.r;
        object.set_face_normal(&ray, &outward_normal);

        object.col = self.col;

        return true;

    }
}