fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {j}");
        for i in 0..IMAGE_WIDTH {
            let r: u32 = ((i as f32 / (IMAGE_WIDTH-1) as f32) as f32 * 255.99) as u32;
            let g: u32 = ((j as f32 / (IMAGE_HEIGHT-1) as f32) as f32 * 255.99) as u32;
            let b: u32 = (0.25 * 255.99) as u32;

            println!("{} {} {}", r, g, b);
        }
    }

    eprintln!("\nDone");
}
