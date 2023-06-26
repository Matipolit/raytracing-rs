use png;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use vector3d::Vector3d;

mod three;

fn render_pixel(color: Vector3d<f64>) -> [u8; 4] {
    return [
        (255.99 * color.x) as u8,
        (255.99 * color.y) as u8,
        (255.99 * color.y) as u8,
        255,
    ];
}

fn render_to_png(width: u32, height: u32, path_str: String) {
    let path = Path::new(&path_str);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    let mut y = height;
    let mut x;
    let mut pixel;
    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

    while y > 0 {
        println!("Lines remaining: {}", y);
        x = 0;
        while x < width {
            pixel = render_pixel(Vector3d::new(
                x as f64 / width as f64,
                y as f64 / height as f64,
                0.25,
            ));
            pixels.extend_from_slice(&pixel);
            x += 1;
        }
        y -= 1;
    }
    writer.write_image_data(&pixels[..]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: u32 = match args[1].parse() {
        Ok(num) => num,
        _ => {
            println!("Width not provided, using 256");
            256
        }
    };

    let height: u32 = match args[2].parse() {
        Ok(num) => num,
        _ => {
            println!("Height not provided, using 256");
            256
        }
    };
    render_to_png(width, height, "image.png".to_string());
}
