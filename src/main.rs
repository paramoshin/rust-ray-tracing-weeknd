use std::f64;
use std::fs::OpenOptions;
use std::io::prelude::*;

use rand::prelude::*;

use ray_tracing::materials::dielectric::Dielectric;
use ray_tracing::materials::lambertian::Lambertian;
use ray_tracing::materials::metal::Metal;
use ray_tracing::materials::{Material, Scatterable};
use ray_tracing::objects::camera::Camera;
use ray_tracing::objects::sphere::Sphere;
use ray_tracing::objects::*;
use ray_tracing::structs::ray::Ray;
use ray_tracing::structs::vec3::Vec3;

fn color(r: &Ray, world: &HittableList, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0001, f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50
            && rec
                .material
                .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_dir: Vec3 = Vec3::unit_vector(&r.direction());
        let t: f64 = 0.5 * (unit_dir.y() + 1.0);
        let v1 = Vec3 {
            d0: 1.0,
            d1: 1.0,
            d2: 1.0,
        };
        let v2 = Vec3 {
            d0: 0.5,
            d1: 0.7,
            d2: 1.0,
        };
        v1 * (1.0 - t) + v2 * t
    }
}

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("img.ppm")
        .unwrap();

    let nx: usize = 200;
    let ny: usize = 100;
    let ns: usize = 100;

    file.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny))?;

    let lookfrom: Vec3 = Vec3::new(3.0, 3.0, 2.0);
    let lookat: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus: f64 = (lookfrom - lookat).length();
    let aperture: f64 = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    let mut world = HittableList::new();
    world.push(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    ));
    world.push(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)),
    ));
    world.push(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(Dielectric::new(1.5)),
    ));
    world.push(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Material::Dielectric(Dielectric::new(1.5)),
    ));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let r1: f64 = random();
                let r2: f64 = random();
                let u: f64 = (i as f64 + r1) / nx as f64;
                let v: f64 = (j as f64 + r2) / ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f64;
            col = Vec3::new(col.d0.sqrt(), col.d1.sqrt(), col.d2.sqrt());
            let ir: i32 = (255.99 * col.d0) as i32;
            let ig: i32 = (255.99 * col.d1) as i32;
            let ib: i32 = (255.99 * col.d2) as i32;
            file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }

    Ok(())
}
