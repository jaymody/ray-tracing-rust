mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use indicatif::ParallelProgressIterator;
use material::{Dielectric, Lambertian, Material, Metal};
use rand::Rng;
use rayon::prelude::*;

use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

const BLACK: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const WHITE: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const SKY_BLUE: Vec3 = Vec3 {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

fn random_scene() -> HittableList {
    let mut world = HittableList::new_empty();

    // ground material
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    // random spheres in the scene
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Box<dyn Material> = if choose_mat < 0.8 {
                    Box::new(Lambertian::new(
                        Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0),
                    ))
                } else if choose_mat < 0.95 {
                    Box::new(Metal::new(
                        Vec3::random(0.5, 1.0),
                        rand::thread_rng().gen_range(0.0..0.5),
                    ))
                } else {
                    Box::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)),
    )));

    world
}

fn ray_color(ray: Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return BLACK;
    }
    match world.hit(ray, 0.001, std::f64::INFINITY) {
        Some(hit) => {
            return {
                let (reflected_ray, attenuation) = hit.material.scatter(ray, &hit);
                ray_color(reflected_ray, world, depth - 1) * attenuation
            }
        }
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = (unit_direction.y + 1.0) * 0.5;
            WHITE * (1.0 - t) + SKY_BLUE * t
        }
    }
}

fn write_color(v: Vec3) {
    let to_val = |x: f64| (x.sqrt().clamp(0.0, 0.999) * 256.0) as u32;
    println!("{} {} {}", to_val(v.x), to_val(v.y), to_val(v.z));
}

fn render(
    image_width: u32,
    image_height: u32,
    camera: Camera,
    world: HittableList,
    samples_per_pixel: u32,
    max_depth: u32,
) {
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let get_u = |i: u32| (i as f64 + rand::thread_rng().gen::<f64>()) / (image_width - 1) as f64;
    let get_v = |j: u32| (j as f64 + rand::thread_rng().gen::<f64>()) / (image_height - 1) as f64;
    let pixel_color = |i: u32, j: u32| {
        (0..samples_per_pixel)
            .into_par_iter()
            .map(|_| ray_color(camera.get_ray(get_u(i), get_v(j)), &world, max_depth))
            .reduce(|| Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
            / samples_per_pixel as f64
    };

    (0..image_height)
        .into_par_iter()
        .rev()
        .progress()
        .flat_map(|j| {
            (0..image_width)
                .into_par_iter()
                .map(move |i| pixel_color(i, j))
                .collect::<Vec<Vec3>>()
        })
        .collect::<Vec<Vec3>>()
        .iter()
        .for_each(|color| write_color(*color));
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, 0.1, 10.0);

    // world
    let world = random_scene();

    // render
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;
    render(
        image_width,
        image_height,
        camera,
        world,
        samples_per_pixel,
        max_depth,
    );
}
