use std::path::Path;

use convolve2d::{DynamicMatrix, Matrix, SubPixels, convolve2d};
use image::RgbImage;

/// Load an image file and return a DynamicMatrix of RGB pixels.
pub fn load_rgb_matrix<P: AsRef<Path>>(
    path: P,
) -> Result<DynamicMatrix<SubPixels<u8, 3>>, image::ImageError> {
    let img: RgbImage = image::io::Reader::open(path)?.decode()?.to_rgb8();
    Ok(img.into())
}

/// Load an image file and return a DynamicMatrix of HSV pixels.
pub fn load_hsv_matrix<P: AsRef<Path>>(
    path: P,
) -> Result<DynamicMatrix<SubPixels<f32, 3>>, image::ImageError> {
    let rgb = load_rgb_matrix(path)?;
    Ok(rgb_matrix_to_hsv(&rgb))
}

/// Average colors in a window using convolution to reduce color detail.
pub fn average_rgb_matrix(
    matrix: &DynamicMatrix<SubPixels<u8, 3>>,
    window: usize,
    stride: usize,
) -> Result<DynamicMatrix<SubPixels<u8, 3>>, String> {
    if window == 0 {
        return Err("window size must be > 0".to_string());
    }
    if stride == 0 {
        return Err("stride must be > 0".to_string());
    }

    let width = matrix.get_width();
    let height = matrix.get_height();
    if width == 0 || height == 0 {
        return Ok(DynamicMatrix::new(0, 0, Vec::new()).unwrap());
    }

    let out_w = width.saturating_sub(window) / stride + 1;
    let out_h = height.saturating_sub(window) / stride + 1;
    let data = matrix.get_data();
    let mut out = Vec::with_capacity(out_w * out_h);

    for oy in 0..out_h {
        for ox in 0..out_w {
            let start_x = ox * stride;
            let start_y = oy * stride;
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;

            for wy in 0..window {
                let y = start_y + wy;
                for wx in 0..window {
                    let x = start_x + wx;
                    let idx = y * width + x;
                    let sp = data[idx].0;
                    sum_r += sp[0] as u32;
                    sum_g += sp[1] as u32;
                    sum_b += sp[2] as u32;
                }
            }

            let denom = (window * window) as u32;
            out.push(SubPixels([
                (sum_r / denom) as u8,
                (sum_g / denom) as u8,
                (sum_b / denom) as u8,
            ]));
        }
    }

    Ok(DynamicMatrix::new(out_w, out_h, out).unwrap())
}

/// Load an image file, then average colors in a window using convolution.
pub fn load_rgb_matrix_averaged<P: AsRef<Path>>(
    path: P,
    window: usize,
    stride: usize,
) -> Result<DynamicMatrix<SubPixels<u8, 3>>, String> {
    let rgb = load_rgb_matrix(path).map_err(|e| e.to_string())?;
    average_rgb_matrix(&rgb, window, stride)
}

/// Convert an RGB DynamicMatrix to HSV (H in degrees 0..360, S/V in 0..1).
pub fn rgb_matrix_to_hsv(
    matrix: &DynamicMatrix<SubPixels<u8, 3>>,
) -> DynamicMatrix<SubPixels<f32, 3>> {
    let width = matrix.get_width();
    let height = matrix.get_height();
    let data = matrix
        .get_data()
        .iter()
        .map(|sp| {
            let [r, g, b] = sp.0;
            SubPixels(rgb_to_hsv([r, g, b]))
        })
        .collect();

    DynamicMatrix::new(width, height, data).unwrap()
}

fn f32_to_u8(value: f32) -> u8 {
    (value * 255.0).round().clamp(0.0, 255.0) as u8
}

fn rgb_to_hsv([r, g, b]: [u8; 3]) -> [f32; 3] {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == rf {
        60.0 * (((gf - bf) / delta) % 6.0)
    } else if max == gf {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    [h, s, v]
}
