use std::path::Path;

/// Load an image file and return a row-major RGB matrix.
pub fn load_rgb_matrix<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Vec<[u8; 3]>>, image::ImageError> {
    let img = image::io::Reader::open(path)?.decode()?.to_rgb8();
    let (width, height) = img.dimensions();

    let mut rows = vec![Vec::with_capacity(width as usize); height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            rows[y as usize].push(pixel);
        }
    }

    Ok(rows)
}
