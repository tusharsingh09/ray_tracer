use std::vec;

use crate::hittable::*;
use crate::ray::*;

pub struct Hittable_List {
    objects: Vec<Box<dyn Hittable>>
}

impl Hittable_List {
    pub fn new() -> Self {
        Hittable_List {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut rec_temp: HitRecord = HitRecord::new();
        let mut hit_any: bool = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(ray, ray_tmin, closest_so_far, &mut rec_temp) {
                hit_any = true;
                closest_so_far = rec_temp.t;
                *rec = rec_temp;

            }
        }

        hit_any
    }
}