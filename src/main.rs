mod vec3;

fn main() {
    let image_width: usize = 256;
    let image_height: usize = 256;

    println!("P3\n{} {}\n255", image_width, image_height);
    for i in 0..image_height {
        for j in 0..image_width {
            let r: f64 = j as f64 / (image_width - 1) as f64;
            let g: f64 = i as f64 / (image_height - 1) as f64;
            let b: f64 = 0.;

            let ir: i8 = (256. * r) as i8;
            let ig: i8 = (256. * g) as i8;
            let ib: i8 = (256. * b) as i8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
