use crate::vector::*;
use crate::ray::*;
use crate::interval::*;
use crate::ppm::*;
use crate::material::*;

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vector,
    pub normal: Vector,
    pub t: f64,
    front_face: bool,
    pub col: Color
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Vector(0.0, 0.0, 0.0),
            normal: Vector(0.0, 0.0, 0.0),
            t: 0.0, front_face: false,
            col: Color(0.0, 0.0, 0.0)
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector) -> () {
        self.front_face = dot(ray.dir(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *outward_normal * -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval, object: &mut HitRecord) -> bool;
}