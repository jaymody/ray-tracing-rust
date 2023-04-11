mod hittable;
mod ray;
mod sphere;
mod vec3;

use hittable::{HitRecord, Hittable};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

// image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

// camera (eye)
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;
const ORIGIN: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const HORIZONTAL: Vec3 = Vec3 {
    x: VIEWPORT_WIDTH,
    y: 0.0,
    z: 0.0,
};
const VERTICAL: Vec3 = Vec3 {
    x: 0.0,
    y: VIEWPORT_HEIGHT,
    z: 0.0,
};
const DEPTH: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: FOCAL_LENGTH,
};

// colors
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

fn ray_color(ray: Ray) -> Vec3 {
    // sphere
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let mut rec = HitRecord::new_empty();
    if sphere.hit(ray, 0.0, 10000.0, &mut rec) {
        return (rec.normal + 1.0) * 0.5;
    }

    // background color
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t: f64 = (unit_direction.y + 1.0) * 0.5;
    WHITE * (1.0 - t) + SKY_BLUE * t
}

fn write_color(v: Vec3) {
    println!(
        "{} {} {}",
        (v.x * 255.999) as u32,
        (v.y * 255.999) as u32,
        (v.z * 255.999) as u32
    );
}

fn main() {
    // test vec3 methods and operators
    test_vec3();

    // compute lower left hand corner
    // TODO: this is not initialized as a global constant because the
    // overloaded operators unfortunately do not work on constants :/
    // I feel like there is probably a way to intelligently do this but
    // I don't know how
    let lower_left_corner: Vec3 = ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - DEPTH;

    // render
    eprintln!("\nStarting render");

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r: Ray = Ray::new(
                ORIGIN,
                lower_left_corner + HORIZONTAL * u + VERTICAL * v - ORIGIN,
            );
            let pixel_color: Vec3 = ray_color(r);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone");
}
