use crate::hittable_list::*;
use crate::ray::*;
use crate::hittable::*;
use crate::interval::*;
use crate::vector::*;
use crate::ppm::*;
use crate::util::*;

#[derive(Default)]
pub struct Camera {
    aspect_ratio: f64,
    width: u16,
    height: u16,
    center: Vector,
    pixel00_loc: Vector,
    delta_u: Vector,
    delta_v: Vector,
    
    // anti aliasing
    samples: u8,
    pixel_samp_scale: f64
}

impl Camera {
    pub fn render(&mut self, world: &Hittable_List) {
        self.initialize();
        let mut writer = init_ppm(self.width, (self.aspect_ratio * (self.width as f64) ) as u16).expect("Failed to init PPM\n");
        for j in 0..self.height {
            for i in 0..self.width {

                /* 
                let pixel_center = self.pixel00_loc + (self.delta_u * i as f64) + (self.delta_v * j as f64); 
                let ray_dir = pixel_center - self.center;
                let r = Ray::new(ray_dir, self.center);
                */
                // let mut pixel_color: Color = self.ray_color(&ray, world);
                let mut pixel_color: Color = Color(0.0, 0.0, 0.0);
                for sample in 0..self.samples {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, world);
                }
                push_pixel(pixel_color * self.pixel_samp_scale, &mut writer).expect("deadbeef");
            }
        }
    }

    pub fn set_aspect_ratio(&mut self, r: f64) {
        self.aspect_ratio = r;
    }
     pub fn set_width(&mut self, width: u16) {
        self.width = width;
     }

    pub fn get_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn set_samples_per_pixel(&mut self, n: u8) {
        self.samples = n;
    }

    fn initialize(&mut self) {
        self.height = (self.width as f64 / self.aspect_ratio) as u16;
        self.center = Vector(0.0, 0.0, 0.0);

        // anti aliasing
        self.pixel_samp_scale = 1.0 / (self.samples as f64);

        // camera and viewport

        let focal_len = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.width as f64) / (self.height as f64);

        let viewport_u = Vector(viewport_width, 0.0, 0.0);
        let viewport_v = Vector(0.0, -viewport_width, 0.0);

        self.delta_u = viewport_u / (self.width as f64);
        self.delta_v = viewport_v / (self.height as f64);

        let viewport_top_left = self.center - Vector(0.0, 0.0, focal_len) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_top_left + (self.delta_u + self.delta_v) * 0.5;
    }

    fn ray_color(&self, r: &Ray, world: &Hittable_List) -> Color{
        let mut rec: HitRecord = HitRecord::new();

        if(world.hit(r, &Interval::new(0.0, f64::INFINITY), &mut rec)) {
            let v = rec.normal + Vector(1.0, 1.0, 1.0);
            return Color(v.0, v.1, v.2) * 0.5;
        }
        
        let unit_dir = unit_vector(r.dir());
        let a = (unit_dir.1 + 1.0) * 0.5;
        return Color(1.0, 1.0, 1.0) * (1.0 - a) + Color(0.5, 0.7, 1.0) * a;
    }

    fn sample_sq() -> Vector {
        Vector(rand() - 0.5, rand() - 0.5, 0.0)
    }

    pub fn get_ray(&self, i: u16, j: u16) -> Ray {
        let offset = Self::sample_sq();
        let pixel_sample = self.pixel00_loc + (self.delta_u * (i as f64 + offset.0)) + ((self.delta_v * (j as f64 + offset.1)));

        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;

        Ray::new(ray_dir, ray_origin)
    }

}