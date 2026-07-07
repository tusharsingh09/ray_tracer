mod ppm;
mod vector;
mod ray;

use std::{fs::File, io::BufWriter};

use ppm::*;
use vector::*;
use ray::*;

fn ray_color(r: &Ray) -> Color {
    let mut col: Color = Color(255, 255, 255);
    let unit_dir: Vector = unit_vector(r.dir());
    let a = (unit_dir.1 + 1.0) * 0.5;
    col = Color(255, 255, 255) * (1.0 - a) + Color(0, 0, 0) * a;

    if (hit_sphere(Vector(0.0, 0.0, -2.0), 0.5, &r)) {
        col = Color(255, 0, 0);
    }

    if hit_sphere(Vector(0.5, 0.5, -1.5), 0.8, &r) {
        col = Color(0, 255, 255);
    }
    col
}

// derived from equation of sphere
// center radius and ray
fn hit_sphere(c: Vector, r: f64, ray: &Ray) -> bool {
    let oc: Vector = c - *ray.origin();
    let a: f64 = dot(ray.dir(), ray.dir());
    let b: f64 = -2.0 * dot(ray.dir(), &oc);
    let c: f64 = dot(&oc, &oc) - r*r;
    let discriminant = b*b - 4.0*a*c;
    discriminant >= 0.0
}

fn main() {
    // use p3, each line will have one pixel
    // going from top left to bottom right
    const ASPECT_RATIO: f64 = 16./9.;
    let height: u16 = 400;
    let width: u16 = (height as f64 * ASPECT_RATIO) as u16;

    // camera settings
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (width as f64) / (height as f64);
    let camera_center: Vector = Vector(0.0, 0.0, 0.0);

    // dimensions for viewport
    let viewport_u: Vector = Vector(viewport_width, 0.0, 0.0);
    let viewport_v: Vector = Vector(0.0, -viewport_height, 0.0);

    // distance between each pixel
    let delta_u: Vector = viewport_u / (width as f64);
    let delta_v: Vector = viewport_v / (height as f64);

    let viewport_top_left: Vector = camera_center - Vector(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel_00_coordinate: Vector = viewport_top_left + delta_u * 0.5 + delta_v * 0.5;

    let mut writer: BufWriter<File> = init_ppm(width, height).expect("Failed");

    for j in 0..height {
        for i in 0..width {
            let pixel_center = pixel_00_coordinate + (delta_u * i as f64) + (delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(ray_direction, camera_center); 

            push_pixel(ray_color(&ray), &mut writer).expect("Failed");
        }
    }
}
