use rand::prelude::*;

use crate::objects::HitRecord;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use self::dielectric::Dielectric;
use self::lambertian::Lambertian;
use self::metal::Metal;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    loop {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}

pub trait Scatterable {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(ref material) => {
                material.scatter(r_in, rec, attenuation, scattered)
            }
            Material::Metal(ref material) => material.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(ref material) => {
                material.scatter(r_in, rec, attenuation, scattered)
            }
        }
    }
}
