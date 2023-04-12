mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use std::io::{self, Write};

use rand::Rng;

use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

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

fn test_vec3() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = Vec3::new(-2.0, 1.0, 2.0);

    eprintln!("{:>20}  =  {}", "u", u);
    eprintln!("{:>20}  =  {}", "v", v);
    eprintln!("{:>20}  =  {}", "-u", -u);
    eprintln!("{:>20}  =  {}", "u + 1.0", u + 1.0);
    eprintln!("{:>20}  =  {}", "u - 1.0", u - 1.0);
    eprintln!("{:>20}  =  {}", "u * -2.0", u * -2.0);
    eprintln!("{:>20}  =  {}", "u / 0.5", u / 0.5);
    eprintln!("{:>20}  =  {}", "u + v", u + v);
    eprintln!("{:>20}  =  {}", "u - v", u - v);
    eprintln!("{:>20}  =  {}", "u * v", u * v);
    eprintln!("{:>20}  =  {}", "u / v", u / v);
    eprintln!("{:>20}  =  {}", "u.squared_length()", u.squared_length());
    eprintln!("{:>20}  =  {}", "u.length()", u.length());
    eprintln!("{:>20}  =  {}", "u.unit_vector()", u.unit_vector());
    eprintln!("{:>20}  =  {}", "u.dot(v)", u.dot(v));
    eprintln!("{:>20}  =  {}", "v.dot(u)", v.dot(u));
    eprintln!("{:>20}  =  {}", "u.cross(v)", u.cross(v));
    eprintln!("{:>20}  =  {}", "v.cross(u)", v.cross(u));
}

fn ray_color(ray: Ray, world: &dyn Hittable) -> Vec3 {
    match world.hit(ray, 0.0, std::f64::INFINITY) {
        Some(hit) => return (hit.normal + 1.0) * 0.5,
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = (unit_direction.y + 1.0) * 0.5;
            WHITE * (1.0 - t) + SKY_BLUE * t
        }
    }
}

fn write_color(v: Vec3) {
    let to_val = |x: f64| (x.clamp(0.0, 0.999) * 256.0) as u32;
    println!("{} {} {}", to_val(v.x), to_val(v.y), to_val(v.z));
}

fn render(
    image_width: u32,
    image_height: u32,
    camera: Camera,
    world: HittableList,
    samples_per_pixel: u32,
) {
    eprintln!("\nStarting render");

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let get_u = |i: u32| (i as f64 + rand::thread_rng().gen::<f64>()) / (image_width - 1) as f64;
    let get_v = |j: u32| (j as f64 + rand::thread_rng().gen::<f64>()) / (image_height - 1) as f64;
    let get_ray = |i: u32, j: u32| camera.get_ray(get_u(i), get_v(j));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining {j}");
        for i in 0..image_width {
            let pixel_color = (0..samples_per_pixel)
                .map(|_| get_ray(i, j))
                .map(|ray| ray_color(ray, &world))
                .reduce(|a, b| a + b)
                .unwrap()
                / samples_per_pixel as f64;
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone render");
}

fn main() {
    // test vec3 methods and operators
    test_vec3();

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
    render(image_width, image_height, camera, world, 100);
}
