mod image_matrix;

use convolve2d::Matrix;
use image::RgbImage;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let path = args
        .next()
        .ok_or("usage: color-scheme-generator <image_path> [--window N]")?;

    let mut window: Option<usize> = None;
    let mut stride: Option<usize> = None;
    while let Some(arg) = args.next() {
        if arg == "--window" || arg == "-w" {
            let value = args
                .next()
                .ok_or("missing value for --window")?
                .parse::<usize>()
                .map_err(|_| "window must be a positive integer")?;
            window = Some(value);
        } else if arg == "--stride" || arg == "-s" {
            let value = args
                .next()
                .ok_or("missing value for --stride")?
                .parse::<usize>()
                .map_err(|_| "stride must be a positive integer")?;
            stride = Some(value);
        } else {
            return Err(format!("unknown argument: {}", arg).into());
        }
    }

    let rgb = if let Some(w) = window {
        let s = stride.unwrap_or(w);
        image_matrix::load_rgb_matrix_averaged(&path, w, s)?
    } else {
        image_matrix::load_rgb_matrix(&path)?
    };
    let hsv = image_matrix::load_hsv_matrix(&path)?;

    let width = rgb.get_width();
    let height = rgb.get_height();
    println!("Loaded RGB matrix: {}x{}", width, height);
    println!(
        "Loaded HSV matrix: {}x{}",
        hsv.get_width(),
        hsv.get_height()
    );
    if let Some(first) = rgb.get_data().first() {
        println!("{:?}", first);
    }
    if let Some(first) = hsv.get_data().first() {
        println!("{:?}", first);
    }

    fs::create_dir_all("data")?;
    let out_path = "data/rgb_output.png";
    let out_image: RgbImage = rgb.into();
    out_image.save(out_path)?;
    println!("Saved RGB image to {}", out_path);
    Ok(())
}
