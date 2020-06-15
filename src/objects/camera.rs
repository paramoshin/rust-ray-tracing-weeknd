use rand::prelude::*;
use std::f64;

use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

fn random_in_unit_disk() -> Vec3 {
    let p: Vec3 = Vec3::new(2.0, 2.0, 2.0);
    let mut rng = rand::thread_rng();
    loop {
        let p: Vec3 =
            Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        if Vec3::dot(&p, &p) < 1.0 {
            break;
        }
    }
    p
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        apeture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta: f64 = vfov * f64::consts::PI / 180.0;
        let half_height: f64 = (theta / 2.0).tan();
        let half_width: f64 = aspect * half_height;
        let w: Vec3 = Vec3::unit_vector(&(lookfrom - lookat));
        let u: Vec3 = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v: Vec3 = Vec3::cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - u * half_width * focus_dist
                - v * half_height * focus_dist
                - w * focus_dist,
            horizontal: u * half_width * focus_dist * 2.0,
            vertical: v * half_height * focus_dist * 2.0,
            lens_radius: apeture / 2.0,
            u: u,
            v: v,
            w: w,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        Ray {
            a: self.origin + offset,
            b: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        }
    }
}
