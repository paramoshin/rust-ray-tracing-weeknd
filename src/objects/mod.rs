use std::f64;

use self::sphere::Sphere;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::Material;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub mod camera;
pub mod sphere;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> Self {
        let lambert = Material::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 0.0)));
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: lambert,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    list: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        let list: Vec<Sphere> = Vec::new();
        HittableList { list: list }
    }

    pub fn push(&mut self, s: Sphere) {
        self.list.push(s);
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.size() {
            if self.list[i].hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hit_record() {
        let hit_record = HitRecord::new();
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_hittable() {
        let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
        let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
        let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
        let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let nx: u64 = 20;
        let ny: u64 = 10;

        let lambert: Lambertian = Lambertian::new(Vec3::new(0.0, 0.0, 0.0));
        let sphere1_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
        let sphere_1: Sphere = Sphere::new(sphere1_center, 0.5, Material::Lambertian(lambert));

        let mut world: HittableList = HittableList::new();
        world.push(sphere_1);

        let mut scene: Vec<Vec<Vec3>> = Vec::new();

        for y in (0..ny).rev() {
            let mut row: Vec<Vec3> = Vec::new();
            for x in 0..nx {
                let u: f64 = x as f64 / nx as f64;
                let v: f64 = y as f64 / ny as f64;
                let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                let mut rec: HitRecord = HitRecord::new();
                if world.hit(&r, 0.0, f64::MAX, &mut rec) {
                    color_vector = Vec3::new(
                        (255.99 * 0.5 * rec.normal.x() + 1.0).floor(),
                        (255.99 * 0.5 * rec.normal.y() + 1.0).floor(),
                        (255.99 * 0.5 * rec.normal.z() + 1.0).floor(),
                    );
                }
                row.push(color_vector);
            }
            scene.push(row);
        }

        assert!(true);
    }
}
