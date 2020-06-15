use std::f64;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Error;

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

fn random_scene() -> HittableList {
    let mut h_list = HittableList::new();
    let sphere0 = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    );
    h_list.push(sphere0);
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center: Vec3 = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    h_list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    ));
                } else {
                    if choose_mat < 0.95 {
                        h_list.push(Sphere::new(
                            center,
                            0.2,
                            Material::Metal(Metal::new(
                                Vec3::new(
                                    0.5 * (1.0 + rng.gen::<f64>()),
                                    0.5 * (1.0 + rng.gen::<f64>()),
                                    0.5 * rng.gen::<f64>(),
                                ),
                                1.0,
                            )),
                        ));
                    } else {
                        h_list.push(Sphere::new(
                            center,
                            0.2,
                            Material::Dielectric(Dielectric::new(1.5)),
                        ));
                    }
                }
            }
        }
    }
    h_list.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric::new(1.5)),
    ));
    h_list.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    ));
    h_list.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ));
    h_list
}

fn color(r: &Ray, world: &HittableList, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth - 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }
    let unit_dir: Vec3 = Vec3::unit_vector(&r.direction());
    let t: f64 = 0.5 * (unit_dir.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn write_color(f: &mut File, pixel_color: Vec3, samples_per_pixel: usize) -> Result<(), Error> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale: f64 = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir: i32 = (255.99 * r) as i32;
    let ig: i32 = (255.99 * g) as i32;
    let ib: i32 = (255.99 * b) as i32;
    f.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("img.ppm")
        .unwrap();

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 1200;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel: usize = 10;
    let max_depth: usize = 50;

    file.write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))?;

    let world = random_scene();
    let lookfrom: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let lookat: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v: f64 = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += color(&r, &world, max_depth);
            }
            write_color(&mut file, pixel_color, samples_per_pixel)?;
        }
    }
    println!("Done");
    Ok(())
}
