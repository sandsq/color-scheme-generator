mod image_matrix;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: color-scheme-generator <image_path>")?;
    let matrix = image_matrix::load_rgb_matrix(path)?;

    let height = matrix.len();
    let width = matrix.first().map(|row| row.len()).unwrap_or(0);
    println!("Loaded RGB matrix: {}x{}", width, height);
    println!("{:?}", matrix[0][0]);
    Ok(())
}
