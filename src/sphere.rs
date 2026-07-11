use crate::{hittable::{self, Hittable}, vector::Vector};
use crate::vector::*;

pub struct Sphere {
    c: Vector,
    r: f64
}

impl Sphere {
    pub fn new(c: Vector, r: f64) -> Self {
        Sphere {c, r}
    }

    pub fn center(&self) -> Vector {
        self.c
    }

    pub fn radius(&self) -> f64 {
        self.r
    }

}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, object: &mut hittable::HitRecord) -> bool {
        let oc = self.c - *ray.origin(); 
        let a = ray.dir().length_squared();
        let h = dot(ray.dir(), &oc);
        let c = oc.length_squared() - self.r * self.r;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 { return false; }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;

        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrt_d) / a;
            if root <= ray_tmin || root >= ray_tmax { return false; }
        }

        object.t = root;
        object.p = ray.at(object.t);
        // object.normal = (object.p - self.c) / self.r;
        let outward_normal: Vector = (object.p - self.c) / self.r;
        object.set_face_normal(&ray, &outward_normal);

        return true;

    }
}