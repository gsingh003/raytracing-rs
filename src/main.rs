mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

use std::sync::{Arc, Mutex};
use std::time::Instant;

use image::{Rgba, RgbaImage};
use rand::Rng;
use rayon::prelude::*;

use rtt::camera::Camera;
use rtt::hittable::{Hittable, HittableList, Sphere};
use rtt::material::{Dielectric, Lambertian, Material, Metal};
use rtt::ray::Ray;
use rtt::vec3::{Color, Point3, Vec3};

const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const BLUE: Color = Color {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

fn clamp_u8(x: f64) -> u8 {
    let x = x.clamp(0.0, 0.999);
    (255.99 * x) as u8
}

fn ray_color(ray: Ray, world: &HittableList, depth: i32, rng: &mut dyn rand::RngCore) -> Color {
    if depth >= 50 {
        return BLACK;
    }

    if let Some(rec) = world.hit(&ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(&ray, &rec, rng) {
            return attenuation * ray_color(scattered, world, depth + 1, rng);
        } else {
            return BLACK;
        }
    }

    let unit_dir = Vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * WHITE + t * BLUE
}

fn random_scene() -> HittableList {
    let mut rng = rand::rng();
    let mut world = HittableList::new();

    let ground_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::new(
                        rng.random::<f64>() * rng.random::<f64>(),
                        rng.random::<f64>() * rng.random::<f64>(),
                        rng.random::<f64>() * rng.random::<f64>(),
                    );
                    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::new(
                        0.5 * (1.0 + rng.random::<f64>()),
                        0.5 * (1.0 + rng.random::<f64>()),
                        0.5 * (1.0 + rng.random::<f64>()),
                    );
                    let fuzz = 0.1;
                    let mat: Arc<dyn Material> = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    // glass
                    let mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    let num_x: u32 = 1920;
    let num_y: u32 = 1080;
    let num_samples: u32 = 10;
    let aspect_ratio = num_x as f64 / num_y as f64;

    let world = random_scene();

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let img = Mutex::new(RgbaImage::new(num_x, num_y));

    let start = Instant::now();

    (0..num_y).into_par_iter().for_each(|j| {
        let mut rng = rand::rng();
        let row = num_y - 1 - j;

        let mut row_pixels: Vec<Rgba<u8>> = Vec::with_capacity(num_x as usize);

        for i in 0..num_x {
            let mut col = Color::new(0.0, 0.0, 0.0);

            for _s in 0..num_samples {
                let u = (i as f64 + rng.random::<f64>()) / num_x as f64;
                let v = (j as f64 + rng.random::<f64>()) / num_y as f64;
                let r = camera.get_ray(u, v, &mut rng);
                col += ray_color(r, &world, 0, &mut rng);
            }

            col /= num_samples as f64;

            // gamma correction
            let col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = clamp_u8(col.r());
            let ig = clamp_u8(col.g());
            let ib = clamp_u8(col.b());

            row_pixels.push(Rgba([ir, ig, ib, 255]));
        }

        {
            let mut img = img.lock().unwrap();
            for (i, px) in row_pixels.into_iter().enumerate() {
                img.put_pixel(i as u32, row, px);
            }
        }

        println!("Scanline {} of {}", num_y - j, num_y);
    });

    let elapsed = start.elapsed();
    println!(
        "Render finished in {:.2} minutes",
        elapsed.as_secs_f64() / 60.0
    );

    let out_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("output.png");

    {
        let img = img.into_inner().expect("image mutex poisoned");
        img.save(&out_path).expect("failed to save image");
    }

    println!("Image saved to: {}", out_path.display());
}
