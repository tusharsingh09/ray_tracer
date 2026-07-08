mod ppm;
mod vector;
mod ray;

use std::{fs::File, io::BufWriter};

use ppm::*;
use vector::*;
use ray::*;

fn ray_color(r: &Ray) -> Color {
    let col: Color; // = Color(255, 255, 255);
    let unit_dir: Vector = unit_vector(r.dir());
    let a = (unit_dir.1 + 1.0) * 0.5;

    let t: f64 = hit_sphere(Vector(0.0, 0.0, -1.0), 0.5, &r);
    print!("{} ", t);
    // println!("{}", t);
    if t > 0.0 {
        // normal to sphere
        let n: Vector = unit_vector(&(r.at(t) - Vector(0.0, 0.0, -1.0)));
        col = Color::new((n.0 + 1.0), (n.1+ 1.0), (n.2 + 1.0)) * 0.5;
        return col;
    }

    col = Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.4 * a, 0.6 * a, 1.0 * a);
    col
}

// derived from equation of sphere
// center radius and ray
fn hit_sphere(c: Vector, r: f64, ray: &Ray) -> f64 {
    let oc: Vector = c - *ray.origin();
    let a: f64 = dot(ray.dir(), ray.dir());
    let b: f64 = -2.0 * dot(ray.dir(), &oc);
    let c: f64 = dot(&oc, &oc) - r*r;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 { -1.0 }
    else { (-b - discriminant.sqrt()) / (2.0 * a) }
}

fn main() {
    // use p3, each line will have one pixel
    // going from top left to bottom right
    const ASPECT_RATIO: f64 = 16.0/9.0;
    let height: u16 = 128;
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

    pixel_00_coordinate.print();

    let mut writer: BufWriter<File> = init_ppm(width, height).expect("Failed");

    for j in 0..height {
        for i in 0..width {
            let pixel_center = pixel_00_coordinate + (delta_u * i as f64) + (delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(ray_direction, camera_center); 
            // println!("{} {} {}", ray.dir().0, ray.dir().1, ray.dir().2);

            push_pixel(ray_color(&ray), &mut writer).expect("Failed");
        }
    }
}
