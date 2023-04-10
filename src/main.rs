mod vec3;

use vec3::Vec3;

fn write_color(v: Vec3) {
    println!(
        "{} {} {}",
        (v.x() * 255.999) as u32,
        (v.y() * 255.999) as u32,
        (v.z() * 255.999) as u32
    );
}

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {j}");
        for i in 0..IMAGE_WIDTH {
            let rgb: Vec3 = Vec3::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            write_color(rgb);
        }
    }
    eprintln!("\nDone");

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
    eprintln!("{:>20}  =  {}", "u.unit()", u.unit());
    eprintln!("{:>20}  =  {}", "u.dot(v)", u.dot(v));
    eprintln!("{:>20}  =  {}", "v.dot(u)", v.dot(u));
    eprintln!("{:>20}  =  {}", "u.cross(v)", u.cross(v));
    eprintln!("{:>20}  =  {}", "v.cross(u)", v.cross(u));
}
