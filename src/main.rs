mod camera;
mod hitrecord;
mod material;
mod object;
mod ray;
mod render;
mod scene;
mod utils;
mod vec3;

use camera::Camera;
use render::render;
use scene::scenes::scene3;
use vec3::Vec3;

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

    // scene
    let scene = scene3();

    // render
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;
    render(
        image_width,
        image_height,
        camera,
        scene,
        samples_per_pixel,
        max_depth,
    );
}
