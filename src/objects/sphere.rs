use super::*;
use crate::materials::Material;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(c: Vec3, r: f64, material: Material) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            material: material,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = Vec3::dot(&r.direction(), &r.direction());
        let b: f64 = Vec3::dot(&oc, &r.direction());
        let c: f64 = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discr: f64 = b * b - a * c;
        if discr > 0.0 {
            let temp: f64 = (-b - discr.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material;
                return true;
            }
        } else {
            let temp = (-b + discr.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material;
                return true;
            }
        }
        return false;
    }
}
