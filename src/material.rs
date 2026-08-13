use crate::hittable::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::ppm::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: Color, scattered: &Ray) -> bool {
        false
    }
}