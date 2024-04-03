use image::DynamicImage;

fn save_with_exit(image: &DynamicImage, path: &str) {
    image.save(path).expect("Failed writing OUTFILE.")
}

fn open_with_exit(path: &str) -> DynamicImage {
    image::open(path).expect("Failed to open INFILE.")
}

pub fn blur(infile: &str, outfile: &str, depth: &f32) {
    let img = open_with_exit(&infile);
    let img_processed = img.blur(*depth);
    save_with_exit(&img_processed, &outfile);
}

pub fn brighten(infile: &str, outfile: &str, value: &i32) {
    let img = open_with_exit(&infile);
    let img_processed = img.brighten(*value);
    save_with_exit(&img_processed, &outfile);
}

pub fn crop(infile: &str, outfile: &str, x: &u32, y: &u32, width: &u32, height: &u32) {
    let mut img = open_with_exit(infile);
    let img_processed = img.crop(*x, *y, *width, *height);
    save_with_exit(&img_processed, outfile);
}

pub fn rotate(infile: &str, outfile: &str, value: &u32) {
    let img = open_with_exit(infile);
    let img_processed: DynamicImage = match *value {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    };
    save_with_exit(&img_processed, outfile);
}

pub fn invert(infile: &str, outfile: &str) {
    let mut img = open_with_exit(infile);
    img.invert();
    save_with_exit(&img, outfile);
}

pub fn grayscale(infile: &str, outfile: &str) {
    let img = open_with_exit(infile);
    let img_processed = img.grayscale();
    save_with_exit(&img_processed, outfile);
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: &str) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
