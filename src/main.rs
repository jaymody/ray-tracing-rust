mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use indicatif::ParallelProgressIterator;
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

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if normal.dot(in_unit_sphere) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

fn ray_color(ray: Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return BLACK;
    }
    match world.hit(ray, 0.001, std::f64::INFINITY) {
        Some(hit) => {
            return ray_color(
                Ray::new(hit.p, random_in_hemisphere(hit.normal)),
                world,
                depth - 1,
            ) * 0.5
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let camera =
        Camera::from_width_height_focal(origin, viewport_width, viewport_height, focal_length);

    // world
    let mut world = HittableList::new_empty();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // render
    let samples_per_pixel: u32 = 100;
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
